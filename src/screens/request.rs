use super::render::handle_screen_defaults;
use crate::app::App;
use crate::display::inputopt::InputOpt;
use crate::display::menuopts::{SAVE_AUTH_ERROR, VALID_COMMAND_ERROR};
use crate::display::AppOptions;
use crate::request::command::CmdType;
use crate::screens::error_alert_box;
use crate::screens::screen::Screen;
use tui::backend::Backend;
use tui::Frame;

pub fn handle_request_menu_screen<B: Backend>(
    app: &mut App,
    frame: &mut Frame<'_, B>,
    err: String,
) {
    handle_screen_defaults(app, frame);
    if !err.is_empty() {
        error_alert_box(frame, &err);
    }
    match app.selected {
        // Add a URL,
        Some(0) => app.goto_screen(Screen::InputMenu(InputOpt::URL(CmdType::Curl))),
        // Add file to upload
        Some(1) => app.goto_screen(Screen::InputMenu(InputOpt::UploadFile)),
        // Add Unix Socket address
        Some(2) => app.goto_screen(Screen::InputMenu(InputOpt::UnixSocket)),
        // Auth
        Some(3) => app.goto_screen(Screen::Authentication),
        // Headers
        Some(4) => app.goto_screen(Screen::Headers),
        // Verbose
        Some(5) => app.add_app_option(AppOptions::Verbose),
        // Request Body
        Some(6) => app.goto_screen(Screen::RequestBodyInput),
        // Save this command
        Some(7) => app.add_app_option(AppOptions::SaveCommand),
        // Save your token or login
        Some(8) => {
            if !app.has_auth() {
                app.goto_screen(Screen::RequestMenu(String::from(SAVE_AUTH_ERROR)));
                return;
            }
            app.add_app_option(AppOptions::SaveToken);
        }
        // Execute command
        Some(9) => {
            if !app.has_url() && !app.has_unix_socket() {
                app.goto_screen(Screen::RequestMenu(String::from(VALID_COMMAND_ERROR)));
                return;
            }
            match app.execute_command() {
                Ok(()) => {
                    let response = app.command.as_mut().unwrap().get_response();
                    app.set_response(&response);
                    app.goto_screen(Screen::Response(response));
                }
                Err(e) => {
                    app.goto_screen(Screen::Error(e.to_string()));
                }
            }
        }
        // more options
        Some(10) => app.goto_screen(Screen::MoreFlags),
        // clear options
        Some(11) => {
            app.remove_all_app_options();
            app.goto_screen(Screen::Method);
        }
        _ => {}
    }
}
