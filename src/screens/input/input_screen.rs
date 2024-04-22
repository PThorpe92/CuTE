use crate::app::App;
use crate::display::menuopts::{
    CERT_ERROR, HEADER_ERROR, PARSE_INT_ERROR, SOCKET_ERROR, UPLOAD_FILEPATH_ERROR,
};
use crate::display::AppOptions;
use crate::request::curl::AuthKind;
use crate::screens::Screen;
use crate::{app::InputMode, display::inputopt::InputOpt};
use std::path::Path;
use tui::prelude::Line;
use tui::style::Color;
use tui::widgets::Paragraph;
use tui::widgets::{Block, Borders};
use tui::{
    prelude::{Constraint, Direction, Layout},
    style::Style,
    text::Text,
    Frame,
};
use tui_input::InputRequest;

pub fn handle_default_input_screen(app: &mut App, frame: &mut Frame<'_>, opt: InputOpt) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(13),
            Constraint::Percentage(9),
            Constraint::Percentage(78),
        ])
        .horizontal_margin(6)
        .split(frame.size());
    // prompt needs to be _directly_ below the input box
    let top_box = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(chunks[1]);
    let width = top_box[0].width.max(3) - 3;
    let scroll = app.input.visual_scroll(width as usize);
    match opt {
        InputOpt::URL => {
            let mut url = app.command.get_url().to_string();
            // if the url has been entered already we populate the input box with it
            // we need to prevent this from happening multiple times, without clearing the url
            if !url.is_empty() && app.input.cursor() == 0 {
                let _ = app.input.handle(InputRequest::InsertChar(' ')).is_some();
                for ch in url.chars() {
                    let _ = app.input.handle(InputRequest::InsertChar(ch)).is_some();
                }
                url.clear();
            }
        }
        InputOpt::Auth(ref kind) => {
            if kind.has_token() {
                let auth = app.command.get_token();
                if auth.is_some() && app.input.value().is_empty() && app.input.cursor() == 0 {
                    let _ = app.input.handle(InputRequest::InsertChar(' ')).is_some();
                    for ch in auth.unwrap().chars() {
                        if app.input.handle(InputRequest::InsertChar(ch)).is_some() {}
                    }
                }
            }
        }
        InputOpt::UploadFile => {
            let file = app.command.get_upload_file();
            if file.is_some() && app.input.value().is_empty() && app.input.cursor() == 0 {
                let _ = app.input.handle(InputRequest::InsertChar(' ')).is_some();
                for ch in file.unwrap().chars() {
                    if app.input.handle(InputRequest::InsertChar(ch)).is_some() {}
                }
            }
        }
        InputOpt::UnixSocket => {
            if !app.command.get_url().is_empty() {
                // would only need a unix socket if we don't have a url
                app.goto_screen(&Screen::RequestMenu(Some(InputOpt::RequestError(
                    String::from("Error: You have already entered a URL"),
                ))));
            }
            let socket = app.command.opts.iter().find_map(|f| {
                if let AppOptions::UnixSocket(s) = f {
                    Some(s)
                } else {
                    None
                }
            });
            if socket.is_some() && app.input.value().is_empty() && app.input.cursor() == 0 {
                let _ = app.input.handle(InputRequest::InsertChar(' ')).is_some();
                for ch in socket.unwrap().chars() {
                    if app.input.handle(InputRequest::InsertChar(ch)).is_some() {}
                }
            }
        }
        InputOpt::CookiePath => {
            if let Some(cookie) = app.command.opts.iter().find_map(|f| {
                if let AppOptions::CookiePath(s) = f {
                    Some(s)
                } else {
                    None
                }
            }) {
                if app.input.value().is_empty() && app.input.cursor() == 0 {
                    let _ = app.input.handle(InputRequest::InsertChar(' ')).is_some();
                    for ch in cookie.chars() {
                        if app.input.handle(InputRequest::InsertChar(ch)).is_some() {}
                    }
                }
            }
        }
        InputOpt::CookieJar => {
            if let Some(cookie) = app.command.opts.iter().find_map(|f| {
                if let AppOptions::CookieJar(s) = f {
                    Some(s)
                } else {
                    None
                }
            }) {
                if app.input.value().is_empty() && app.input.cursor() == 0 {
                    let _ = app.input.handle(InputRequest::InsertChar(' ')).is_some();
                    for ch in cookie.chars() {
                        if app.input.handle(InputRequest::InsertChar(ch)).is_some() {}
                    }
                }
            }
        }
        _ => {}
    }
    let input = Paragraph::new(app.input.value())
        .style(match app.input_mode {
            InputMode::Normal => Style::default().fg(Color::Blue),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            Line::from("Press 'i' to start editing"),
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(tui::style::Modifier::BOLD),
        ),
        InputMode::Editing => (
            Line::from("Press Enter to submit"),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(tui::style::Modifier::BOLD),
        ),
    };
    let msg = Paragraph::new(msg).style(style);
    frame.render_widget(msg, top_box[0]);
    frame.render_widget(input, top_box[1]);
    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => frame.set_cursor(
            top_box[1].x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            top_box[1].y + 1,
        ),
    }
    // we have input (the user has typed something and pressed Enter while in insert mode)
    if !app.messages.is_empty() {
        app.input_mode = InputMode::Normal;
        // parse the input message with the opt to find out what to do with it
        parse_input(app.messages[0].clone(), opt, app);
        app.messages.remove(0);
    }
}

fn is_valid_unix_socket_path(path: &str) -> Result<(), String> {
    let path = Path::new(path);
    if path.is_absolute() || path.starts_with("~") {
        // Ensure it's a Unix socket file (ends with `.sock` or has no extension)
        if let Some(file_name) = path.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                if file_name_str.ends_with(".sock") || !file_name_str.contains('.') {
                    return Ok(());
                }
            }
        }
        Err(SOCKET_ERROR.to_string())
    } else {
        Err(SOCKET_ERROR.to_string())
    }
}

pub fn parse_input(message: String, opt: InputOpt, app: &mut App) {
    match opt {
        InputOpt::URL => {
            app.add_app_option(AppOptions::URL(message));
        }
        InputOpt::ApiKey => {
            let _ = app.db.as_ref().add_key(&message);
        }
        InputOpt::UnixSocket => {
            if let Err(e) = is_valid_unix_socket_path(&message) {
                app.goto_screen(&Screen::RequestMenu(Some(InputOpt::RequestError(e))));
            } else {
                app.add_app_option(AppOptions::UnixSocket(message.clone()));
            }
        }
        InputOpt::Headers => {
            if !validate_key_val(&message) {
                app.goto_screen(&Screen::RequestMenu(Some(InputOpt::RequestError(
                    String::from(HEADER_ERROR),
                ))));
            } else {
                app.add_app_option(AppOptions::Headers(message.clone()));
            }
        }
        InputOpt::RenameCollection(ref id) => {
            if app.db.as_ref().rename_collection(*id, &message).is_ok() {
            } else {
                app.goto_screen(&Screen::Error("Failed to rename collection".to_string()));
            }
        }
        InputOpt::Output => {
            app.add_app_option(AppOptions::Outfile(message));
        }
        InputOpt::CookiePath => {
            app.add_app_option(AppOptions::CookiePath(message));
        }
        InputOpt::CookieJar => {
            app.add_app_option(AppOptions::CookieJar(message));
        }
        InputOpt::NewCookie => {
            app.goto_screen(&Screen::RequestMenu(Some(InputOpt::CookieValue(message))));
        }
        InputOpt::CmdDescription(id) => {
            let coll_id = app
                .db
                .set_command_description(id, &message)
                .unwrap_or_default();
            app.goto_screen(&Screen::SavedCommands {
                id: coll_id,
                opt: Some(InputOpt::RequestError(String::from("Description Updated"))),
            });
        }
        InputOpt::CollectionDescription(id) => {
            app.db
                .set_collection_description(id, &message)
                .unwrap_or_default();
            app.goto_screen(&Screen::SavedCollections(Some(InputOpt::RequestError(
                String::from("Description Updated"),
            ))));
        }
        InputOpt::CookieValue(ref name) => {
            let cookie = format!("{}={};", name, message);
            app.goto_screen(&Screen::RequestMenu(Some(InputOpt::CookieExpires(cookie))));
        }
        InputOpt::CookieExpires(ref cookie) => {
            let cookie = format!("{} {}", cookie, message);
            app.add_app_option(AppOptions::NewCookie(cookie));
        }
        InputOpt::Referrer => {
            app.add_app_option(AppOptions::Referrer(message.clone()));
        }
        InputOpt::CaPath => {
            if !validate_path(&message) {
                app.goto_screen(&Screen::RequestMenu(Some(InputOpt::RequestError(
                    String::from(CERT_ERROR),
                ))));
            } else {
                app.add_app_option(AppOptions::CaPath(message.clone()));
            }
        }
        InputOpt::UserAgent => {
            app.add_app_option(AppOptions::UserAgent(message.clone()));
        }
        InputOpt::MaxRedirects => {
            if let Ok(num) = message.parse::<usize>() {
                app.add_app_option(AppOptions::MaxRedirects(num));
            } else {
                app.goto_screen(&Screen::RequestMenu(Some(InputOpt::RequestError(
                    String::from(PARSE_INT_ERROR),
                ))));
            }
        }
        InputOpt::UploadFile => {
            if !validate_path(&message) {
                app.goto_screen(&Screen::RequestMenu(Some(InputOpt::RequestError(
                    String::from(UPLOAD_FILEPATH_ERROR),
                ))));
            }
            app.add_app_option(AppOptions::UploadFile(message));
        }
        InputOpt::Execute => {
            // This means they have executed the HTTP Request, and want to write to a file
            app.command.set_outfile(&message);
            if let Err(e) = app.command.write_output() {
                app.goto_screen(&Screen::Response(e.to_string()));
            } else {
                app.goto_screen(&Screen::Response(String::from(
                    app.response.as_ref().unwrap_or(&String::new()).as_str(),
                )));
            }
        }
        InputOpt::RequestBody => {
            // if the body is a path to a file, we need to read the file and set the body
            // otherwise we just set the body
            if Path::new(&message).exists() {
                match std::fs::read_to_string(&message) {
                    Ok(body) => {
                        app.add_app_option(AppOptions::RequestBody(body));
                    }
                    Err(e) => app.goto_screen(&Screen::RequestMenu(Some(InputOpt::AlertMessage(
                        e.to_string(),
                    )))),
                }
            } else {
                app.add_app_option(AppOptions::RequestBody(message.clone()));
            }
        }
        InputOpt::ImportCollection => {
            if let Err(e) = app.import_postman_collection(&message) {
                app.goto_screen(&Screen::SavedCollections(Some(InputOpt::AlertMessage(
                    e.to_string(),
                ))));
            } else {
                app.goto_screen(&Screen::SavedCollections(Some(InputOpt::AlertMessage(
                    String::from("Collection Imported"),
                ))));
            }
        }
        InputOpt::KeyLabel(id) => match app.db.set_key_label(id, &message) {
            Ok(_) => app.goto_screen(&Screen::SavedKeys(Some(InputOpt::AlertMessage(
                String::from("Label Updated"),
            )))),
            Err(e) => app.goto_screen(&Screen::SavedKeys(Some(InputOpt::RequestError(format!(
                "Error: {}",
                e
            ))))),
        },
        InputOpt::CmdLabel(id) => match app.db.set_command_label(id, &message) {
            Ok(collection_id) => app.goto_screen(&Screen::SavedCommands {
                id: collection_id,
                opt: Some(InputOpt::AlertMessage(String::from("Label Updated"))),
            }),
            Err(e) => app.goto_screen(&Screen::SavedCommands {
                id: None,
                opt: Some(InputOpt::RequestError(format!("Error: {}", e))),
            }),
        },
        InputOpt::Auth(ref auth) => {
            parse_auth(auth, app, &message);
        }
        _ => {}
    }
    app.goto_screen(&opt.get_return_screen());
}

pub fn render_input_with_prompt<'a, T: Into<Text<'a>>>(frame: &mut Frame<'_>, prompt: T) {
    // Render the input with the provided prompt
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(frame.size());
    let message = Paragraph::new(prompt);
    frame.render_widget(message, chunks[0]);
}

fn validate_key_val(key_val: &str) -> bool {
    let split = key_val.split(':');
    split.count() > 1
}

fn validate_path(path: &str) -> bool {
    Path::new(path).exists()
}

fn parse_auth(auth: &AuthKind, app: &mut App, message: &str) {
    app.command.set_auth(match auth {
        AuthKind::Basic(_) => AuthKind::Basic(String::from(message)),
        AuthKind::Bearer(_) => AuthKind::Bearer(String::from(message)),
        AuthKind::Digest(_) => AuthKind::Digest(String::from(message)),
        // above are the only auth options that would ever send us here
        _ => AuthKind::None,
    });
    if app.command.has_auth() {
        app.command
            .opts
            .retain(|x| !matches!(x, AppOptions::Auth(_)));
    }
    app.command.opts.push(AppOptions::Auth(match auth {
        AuthKind::Basic(_) => AuthKind::Basic(String::from(message)),
        AuthKind::Bearer(_) => AuthKind::Bearer(String::from(message)),
        AuthKind::Digest(_) => AuthKind::Digest(String::from(message)),
        // above are the only auth options that would ever send us here
        _ => AuthKind::None,
    }));
    app.goto_screen(&Screen::RequestMenu(None));
}
