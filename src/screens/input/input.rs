use crate::app::App;
use crate::display::menuopts::{
    CERT_ERROR, HEADER_ERROR, INPUT_OPT_AUTH_ANY, INPUT_OPT_AUTH_BASIC, INPUT_OPT_AUTH_BEARER,
    INPUT_OPT_BASIC, INPUT_OPT_HEADERS, PARSE_INT_ERROR, SOCKET_ERROR, UPLOAD_FILEPATH_ERROR,
};
use crate::display::AppOptions;
use crate::request::curl::AuthKind;
use crate::screens::auth::AuthType;
use crate::screens::{centered_rect, Screen};
use crate::{app::InputMode, display::inputopt::InputOpt};
use std::path::Path;
use tui::prelude::Line;
use tui::style::{Color, Modifier};
use tui::widgets::Paragraph;
use tui::widgets::{Block, Borders};
use tui::{
    prelude::{Constraint, Direction, Layout},
    style::Style,
    text::Text,
    Frame,
};
use tui_input::InputRequest;

// Takes the current option and returns a prompt for that screen
pub fn get_input_prompt(opt: InputOpt) -> Text<'static> {
    match opt {
        InputOpt::URL => Text::from(Line::from("Enter a URL for your request and press Enter")),
        InputOpt::RequestBody => Text::from("Enter a body for your request and press Enter"),
        InputOpt::Headers => Text::from(Line::from(INPUT_OPT_HEADERS)),
        InputOpt::Auth(auth) => match auth {
            AuthType::Basic => Text::from(INPUT_OPT_AUTH_BASIC),
            AuthType::Bearer => Text::from(INPUT_OPT_AUTH_BEARER),
            _ => Text::from(INPUT_OPT_AUTH_ANY),
        },
        InputOpt::CookiePath => Text::from("Enter the path to the cookie jar file"),
        InputOpt::NewCookie => Text::from("Enter the name of the cookie"),
        InputOpt::CookieValue(ref name) => Text::from(format!("Enter the value for {}", name)),
        InputOpt::CookieExpires(_) => {
            Text::from("Enter the (optional) expiration date for the cookie")
        }
        InputOpt::ImportCollection => Text::from("Enter the path to the collection file.json"),
        _ => Text::from(INPUT_OPT_BASIC),
    }
}

pub fn handle_default_input_screen(app: &mut App, frame: &mut Frame<'_>, opt: InputOpt) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(15), Constraint::Percentage(85)])
        .split(frame.size());
    let input_textbox = chunks[0];
    let text_chunk = Block::default().borders(Borders::ALL).style(
        Style::default()
            .bg(app.config.get_bg_color())
            .fg(Color::LightGreen)
            .add_modifier(tui::style::Modifier::BOLD),
    );
    frame.render_widget(text_chunk, input_textbox);
    let bottom_box = centered_rect(chunks[1], crate::screens::ScreenArea::Top);
    let prompt = get_input_prompt(opt.clone());
    frame.render_widget(
        Paragraph::new(prompt).style(
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ),
        centered_rect(bottom_box, crate::screens::ScreenArea::Top),
    );
    let top_box = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(input_textbox);
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
        InputOpt::Auth(kind) => {
            if kind == AuthType::Basic || kind == AuthType::Bearer {
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
            let socket = app.command.get_unix_socket();
            if socket.is_some() && app.input.value().is_empty() && app.input.cursor() == 0 {
                let _ = app.input.handle(InputRequest::InsertChar(' ')).is_some();
                for ch in socket.unwrap().chars() {
                    if app.input.handle(InputRequest::InsertChar(ch)).is_some() {}
                }
            }
        }
        InputOpt::CookiePath => {
            if let Some(cookie) = app.command.get_cookie_path() {
                if app.input.value().is_empty() && app.input.cursor() == 0 {
                    let _ = app.input.handle(InputRequest::InsertChar(' ')).is_some();
                    for ch in cookie.chars() {
                        if app.input.handle(InputRequest::InsertChar(ch)).is_some() {}
                    }
                }
            }
        }
        InputOpt::CookieJar => {
            if let Some(cookie) = app.command.get_cookie_jar_path() {
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
            InputMode::Normal => Style::default().fg(app.config.get_body_color()),
            InputMode::Editing => Style::default().fg(Color::White),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            Line::from("Press 'i' to start editing"),
            Style::default()
                .fg(Color::Green)
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
            app.goto_screen(&Screen::RequestMenu(None));
        }
        InputOpt::ApiKey => {
            let _ = app.add_saved_key(message.clone());
            app.goto_screen(&Screen::SavedKeys);
        }
        InputOpt::UnixSocket => {
            if let Err(e) = is_valid_unix_socket_path(&message) {
                app.goto_screen(&Screen::RequestMenu(Some(InputOpt::RequestError(e))));
            } else {
                app.add_app_option(AppOptions::UnixSocket(message.clone()));
                app.goto_screen(&Screen::RequestMenu(None));
            }
        }
        InputOpt::Headers => {
            if !validate_key_val(&message) {
                app.goto_screen(&Screen::RequestMenu(Some(InputOpt::RequestError(
                    String::from(HEADER_ERROR),
                ))));
            } else {
                app.add_app_option(AppOptions::Headers(message.clone()));
                app.goto_screen(&Screen::RequestMenu(None));
            }
        }
        InputOpt::RenameCollection(ref id) => {
            if app.rename_collection(*id, &message).is_ok() {
                app.goto_screen(&Screen::SavedCollections);
            } else {
                app.goto_screen(&Screen::Error("Failed to rename collection".to_string()));
            }
        }
        InputOpt::Output => {
            app.add_app_option(AppOptions::Outfile(message));
            app.goto_screen(&Screen::RequestMenu(None));
        }
        InputOpt::CookiePath => {
            app.add_app_option(AppOptions::CookiePath(message));
            app.goto_screen(&Screen::RequestMenu(None));
        }
        InputOpt::CookieJar => {
            app.add_app_option(AppOptions::CookieJar(message));
            app.goto_screen(&Screen::RequestMenu(None));
        }
        InputOpt::NewCookie => {
            app.goto_screen(&Screen::RequestMenu(Some(InputOpt::CookieValue(message))));
        }
        InputOpt::CookieValue(ref name) => {
            let cookie = format!("{}={};", name, message);
            app.goto_screen(&Screen::RequestMenu(Some(InputOpt::CookieExpires(cookie))));
        }
        InputOpt::CookieExpires(ref cookie) => {
            let cookie = format!("{} {}", cookie, message);
            app.add_app_option(AppOptions::NewCookie(cookie));
            app.goto_screen(&Screen::RequestMenu(None))
        }
        InputOpt::Referrer => {
            app.add_app_option(AppOptions::Referrer(message.clone()));
            app.goto_screen(&Screen::RequestMenu(None));
        }
        InputOpt::CaPath => {
            if !validate_path(&message) {
                app.goto_screen(&Screen::RequestMenu(Some(InputOpt::RequestError(
                    String::from(CERT_ERROR),
                ))));
            } else {
                app.add_app_option(AppOptions::CaPath(message.clone()));
                app.goto_screen(&Screen::RequestMenu(None));
            }
        }
        InputOpt::UserAgent => {
            app.add_app_option(AppOptions::UserAgent(message.clone()));
            app.goto_screen(&Screen::RequestMenu(None));
        }
        InputOpt::MaxRedirects => {
            if let Ok(num) = message.parse::<usize>() {
                app.add_app_option(AppOptions::MaxRedirects(num));
                app.goto_screen(&Screen::RequestMenu(None));
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
            app.goto_screen(&Screen::RequestMenu(None));
        }
        InputOpt::Execute => {
            // This means they have executed the HTTP Request, and want to write to a file
            app.command.set_outfile(&message);
            if let Err(e) = app.command.write_output() {
                app.goto_screen(&Screen::Error(e.to_string()));
            } else {
                app.goto_screen(&Screen::Response(String::from(app.get_response())));
            }
        }
        InputOpt::RequestBody => {
            app.add_app_option(AppOptions::RequestBody(message.clone()));
            app.goto_screen(&Screen::RequestMenu(None));
        }
        InputOpt::ImportCollection => {
            if let Err(e) = app.import_postman_collection(&message) {
                app.goto_screen(&Screen::Error(e.to_string()));
            } else {
                app.goto_screen(&Screen::Success);
            }
        }
        InputOpt::CreateCollection => {
            if app.create_postman_collection(&message).is_ok() {
                app.goto_screen(&Screen::Success);
                return;
            }
            app.goto_screen(&Screen::Error("Failed to create collection".to_string()));
        }
        InputOpt::KeyLabel(id) => match app.set_key_label(id, &message) {
            Ok(_) => app.goto_screen(&Screen::SavedKeys),
            Err(e) => app.goto_screen(&Screen::Error(e)),
        },
        InputOpt::Auth(auth) => {
            parse_auth(auth, app, &message);
        }
        _ => {}
    }
}

pub fn render_input_with_prompt(frame: &mut Frame<'_>, prompt: Text) {
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

fn parse_auth(auth: AuthType, app: &mut App, message: &str) {
    app.command.set_auth(match auth {
        AuthType::Basic => AuthKind::Basic(String::from(message)),
        AuthType::Bearer => AuthKind::Bearer(String::from(message)),
        AuthType::Digest => AuthKind::Digest(String::from(message)),
        // above are the only auth options that would ever send us here
        _ => AuthKind::None,
    });
    if app.command.has_auth() {
        app.opts.retain(|x| !matches!(x, AppOptions::Auth(_)));
    }
    app.opts.push(AppOptions::Auth(match auth {
        AuthType::Basic => AuthKind::Basic(String::from(message)),
        AuthType::Bearer => AuthKind::Bearer(String::from(message)),
        AuthType::Digest => AuthKind::Digest(String::from(message)),
        // above are the only auth options that would ever send us here
        _ => AuthKind::None,
    }));
    app.goto_screen(&Screen::RequestMenu(None));
}
