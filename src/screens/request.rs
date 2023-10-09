use tui::backend::Backend;
use tui::Frame;

use crate::app::App;
use crate::display::inputopt::InputOpt;
use crate::display::DisplayOpts;
use crate::request::cmdtype::CmdType;
use crate::screens::screen::Screen;

use super::render::handle_screen_defaults;

pub fn handle_request_menu_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    handle_screen_defaults(app, frame);
    match app.selected {
        // Add a URL,
        Some(0) => app.goto_screen(Screen::InputMenu(InputOpt::URL(CmdType::Curl))),
        // Add Unix Socket address
        Some(1) => app.goto_screen(Screen::InputMenu(InputOpt::UnixSocket)),
        // Auth
        Some(2) => app.goto_screen(Screen::Authentication),
        // Headers
        Some(3) => app.goto_screen(Screen::InputMenu(InputOpt::Headers)),
        // Verbose
        Some(4) => {
            app.add_display_option(DisplayOpts::Verbose);
            app.selected = None;
        }
        // Request Body
        Some(5) => {
            app.goto_screen(Screen::InputMenu(InputOpt::RequestBody));
            app.selected = None;
        }
        // Save this command
        Some(6) => {
            app.add_display_option(DisplayOpts::SaveCommand);
            app.selected = None;
        }
        // Save your token or login
        Some(7) => {
            app.add_display_option(DisplayOpts::SaveToken);
            app.selected = None;
        }
        // Execute command
        Some(8) => match app.execute_command() {
            Ok(()) => {
                if app.command.as_ref().unwrap().get_response().is_some() {
                    app.response = app.command.as_ref().unwrap().get_response().clone();
                    app.goto_screen(Screen::Response(app.response.clone().unwrap()));
                    app.selected = None;
                } else {
                    app.goto_screen(Screen::Error("Unable To Retreve Response...".to_string()));
                }
            }
            Err(e) => {
                app.goto_screen(Screen::Error(e.to_string()));
            }
        },

        _ => {}
    }
}
