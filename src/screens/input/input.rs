use crate::app::App;
use crate::display::menuopts::{
    CERT_ERROR, HEADER_ERROR, INPUT_OPT_AUTH_ANY, INPUT_OPT_AUTH_BASIC, INPUT_OPT_AUTH_BEARER,
    INPUT_OPT_BASIC, INPUT_OPT_HEADERS, INPUT_OPT_REC_DOWNLOAD, PARSE_INT_ERROR, SOCKET_ERROR,
    UPLOAD_FILEPATH_ERROR,
};
use crate::display::AppOptions;
use crate::request::command::CmdType;
use crate::request::curl::AuthKind;
use crate::screens::auth::AuthType;
use crate::screens::Screen;
use crate::{app::InputMode, display::inputopt::InputOpt};
use std::path::Path;
use tui::prelude::Line;
use tui::style::Color;
use tui::widgets::Paragraph;
use tui::widgets::{Block, Borders};
use tui::{
    prelude::{Backend, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Text},
    Frame,
};

// Takes the current option and returns a prompt for that screen
pub fn get_input_prompt(opt: InputOpt) -> Text<'static> {
    match opt {
        InputOpt::URL(opt) => {
            let fmtstr = format!("Enter a URL for your {}\n and press Enter", opt);
            Text::from(Line::from(fmtstr))
        }
        InputOpt::RequestBody => Text::from("Enter a body for your request and press Enter"),
        InputOpt::Headers => Text::from(Line::from(INPUT_OPT_HEADERS)),
        InputOpt::RecursiveDownload => Text::from(INPUT_OPT_REC_DOWNLOAD),
        InputOpt::Auth(auth) => match auth {
            AuthType::Basic => Text::from(INPUT_OPT_AUTH_BASIC),
            AuthType::Bearer => Text::from(INPUT_OPT_AUTH_BEARER),
            _ => Text::from(INPUT_OPT_AUTH_ANY),
        },
        _ => Text::from(INPUT_OPT_BASIC),
    }
}

pub fn handle_default_input_screen<B: Backend>(
    app: &mut App,
    frame: &mut Frame<'_, B>,
    opt: InputOpt,
) {
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
    let (_msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press h"),
                Span::raw("to go back."),
                Span::styled("Press i", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("to start editing."),
            ],
            Style::default(),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to submit."),
            ],
            Style::default(),
        ),
    };
    let mut prompt = get_input_prompt(opt.clone());
    prompt.patch_style(style);
    render_input_with_prompt(frame, prompt);

    let width = chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor
    let scroll = app.input.visual_scroll(width as usize);
    let input = Paragraph::new(app.input.value())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::LightBlue),
        })
        .scroll((0, scroll as u16))
        .block(Block::default().borders(Borders::ALL).title("Input"));
    frame.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => frame.set_cursor(
            chunks[1].x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            chunks[1].y + 1,
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
        InputOpt::URL(opt) => {
            match opt {
                CmdType::Wget => {
                    app.add_app_option(AppOptions::URL(message));
                    app.goto_screen(Screen::Downloads("".to_string()));
                }
                CmdType::Curl => {
                    app.add_app_option(AppOptions::URL(message));
                    app.goto_screen(Screen::RequestMenu(String::new()));
                }
            };
        }
        InputOpt::ApiKey => {
            let _ = app.add_saved_key(message.clone());
            app.goto_screen(Screen::SavedKeys);
        }
        InputOpt::UnixSocket => {
            if let Err(e) = is_valid_unix_socket_path(&message) {
                app.goto_screen(Screen::RequestMenu(e));
            } else {
                app.add_app_option(AppOptions::UnixSocket(message.clone()));
                app.goto_screen(Screen::RequestMenu(String::new()));
            }
        }
        InputOpt::Headers => {
            if !validate_key_val(&message) {
                app.goto_screen(Screen::RequestMenu(String::from(HEADER_ERROR)));
            } else {
                app.add_app_option(AppOptions::Headers(message.clone()));
                app.goto_screen(Screen::RequestMenu(String::new()));
            }
        }
        // Only downloads let you specify the output file prior to execution of the command
        InputOpt::Output => {
            app.add_app_option(AppOptions::Outfile(message.clone()));
            app.goto_screen(Screen::Downloads("".to_string()));
        }
        InputOpt::Cookie => {
            app.add_app_option(AppOptions::Cookie(message.clone()));
            app.goto_screen(Screen::RequestMenu(String::new()));
        }
        InputOpt::Referrer => {
            app.add_app_option(AppOptions::Referrer(message.clone()));
            app.goto_screen(Screen::RequestMenu(String::new()));
        }
        InputOpt::CaPath => {
            if !validate_path(&message) {
                app.goto_screen(Screen::RequestMenu(String::from(CERT_ERROR)));
            } else {
                app.add_app_option(AppOptions::CaPath(message.clone()));
                app.goto_screen(Screen::RequestMenu(String::new()));
            }
        }
        InputOpt::UserAgent => {
            app.add_app_option(AppOptions::UserAgent(message.clone()));
            app.goto_screen(Screen::RequestMenu(String::new()));
        }
        InputOpt::MaxRedirects => {
            if let Ok(num) = message.parse::<usize>() {
                app.add_app_option(AppOptions::MaxRedirects(num));
                app.goto_screen(Screen::RequestMenu(String::new()));
            } else {
                app.goto_screen(Screen::RequestMenu(String::from(PARSE_INT_ERROR)));
            }
        }
        InputOpt::UploadFile => {
            if !validate_path(&message) {
                app.goto_screen(Screen::RequestMenu(String::from(UPLOAD_FILEPATH_ERROR)));
            }
            app.add_app_option(AppOptions::UploadFile(message));
            app.goto_screen(Screen::RequestMenu(String::new()));
        }
        InputOpt::Execute => {
            // This means they have executed the HTTP Request, and want to write to a file
            app.command.as_mut().unwrap().set_outfile(&message);
            if let Err(e) = app.command.as_mut().unwrap().write_output() {
                app.goto_screen(Screen::Error(e.to_string()));
            } else {
                app.goto_screen(Screen::Response(String::from(app.get_response())));
            }
        }
        InputOpt::RequestBody => {
            app.add_app_option(AppOptions::RequestBody(message.clone()));
            app.goto_screen(Screen::RequestMenu(String::new()));
        }
        InputOpt::RecursiveDownload => {
            if let Ok(recursion_level) = message.parse::<usize>() {
                app.add_app_option(AppOptions::RecDownload(recursion_level));
                app.goto_screen(Screen::Downloads("".to_string()));
            } else {
                app.goto_screen(Screen::Downloads(String::from(PARSE_INT_ERROR)));
            }
        }
        InputOpt::Auth(auth) => {
            parse_auth(auth, app, &message);
        }
        _ => {}
    }
}

pub fn render_input_with_prompt<B: Backend>(frame: &mut Frame<'_, B>, prompt: Text) {
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
    if app.has_app_option(&AppOptions::Auth(String::new())) {
        app.remove_app_option(&AppOptions::Auth(String::new()));
    }
    app.command.as_mut().unwrap().set_auth(match auth {
        AuthType::Basic => AuthKind::Basic(String::from(message)),
        AuthType::Bearer => AuthKind::Bearer(String::from(message)),
        AuthType::Digest => AuthKind::Digest(String::from(message)),
        AuthType::AWSSignatureV4 => AuthKind::AwsSigv4,
        AuthType::SPNEGO => AuthKind::Spnego,
        AuthType::NTLM => AuthKind::Ntlm,
    });
    app.add_app_option(AppOptions::Auth(String::from(message)));
    app.goto_screen(Screen::RequestMenu(String::new()));
}
