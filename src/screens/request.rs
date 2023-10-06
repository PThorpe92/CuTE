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
        Some(num) => match num {
            // Add a URL,
            0 => app.goto_screen(Screen::InputMenu(InputOpt::URL(CmdType::Curl))),
            // Auth
            1 => app.goto_screen(Screen::Authentication),
            // Headers
            2 => app.goto_screen(Screen::InputMenu(InputOpt::Headers)),
            // Verbose
            3 => {
                if app.opts.contains(&&DisplayOpts::Verbose) {
                    app.opts.retain(|x| *x != DisplayOpts::Verbose);
                    app.command.as_mut().unwrap().set_verbose(false);
                } else {
                    app.add_display_option(DisplayOpts::Verbose);
                    app.command.as_mut().unwrap().set_verbose(true);
                }
                app.selected = None;
            }
            // Request Body
            4 => {
                app.goto_screen(Screen::InputMenu(InputOpt::RequestBody));
                app.selected = None;
            }
            // Save this command
            5 => {
                // we want to lazy load the db connection. so we
                // dont actually establish the connection until we know
                // we are actually goign to store the command or look something up.
                if app.has_display_option(&DisplayOpts::SaveCommand) {
                    app.remove_display_option(&DisplayOpts::SaveCommand);
                    app.command.as_mut().unwrap().save_command(false);
                } else {
                    app.command.as_mut().unwrap().save_command(true);
                    app.add_display_option(DisplayOpts::SaveCommand);
                }
                app.selected = None;
            }
            // Save your token or login
            6 => {
                app.add_display_option(DisplayOpts::SaveToken);
                app.selected = None;
            }
            //
            // Execute command
            7 => match app.execute_command() {
                Ok(()) => {
                    if app.command.as_ref().unwrap().get_response().is_some() {
                        app.response = app.command.as_ref().unwrap().get_response().clone();
                        app.goto_screen(Screen::Response(app.response.clone().unwrap()));
                    } else {
                        app.goto_screen(Screen::Error("Unable To Retreve Response...".to_string()));
                    }
                }
                Err(e) => {
                    app.goto_screen(Screen::Error(e.to_string()));
                }
            },

            _ => {}
        },
        None => {}
    }
}
