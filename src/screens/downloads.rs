use crate::app::App;
use crate::display::inputopt::InputOpt;
use crate::request::cmdtype::CmdType;
use crate::request::command::Command;
use crate::request::wget::Wget;
use crate::screens::screen::Screen;
use tui::backend::Backend;
use tui::Frame;

use super::render::handle_screen_defaults;

pub fn handle_downloads_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    if app.command.is_none() {
        app.set_command(Command::Wget(Wget::new()));
    }
    handle_screen_defaults(app, frame);
    match app.selected {
        // Setting Recursion level
        Some(0) => {
            app.goto_screen(Screen::InputMenu(InputOpt::RecursiveDownload));
            app.selected = None;
        }
        // Add URL of download
        Some(1) => {
            app.goto_screen(Screen::InputMenu(InputOpt::URL(CmdType::Wget)));
            app.selected = None;
        }
        // Add file name for output/download
        Some(2) => {
            app.goto_screen(Screen::InputMenu(InputOpt::Output));
            app.selected = None;
            // Execute command
        }
        Some(3) => match app.execute_command() {
            Ok(_) => {
                if let Some(response) = app.command.as_ref().unwrap().get_response() {
                    app.response = Some(response.clone());
                    app.goto_screen(Screen::Response(response));
                }
            }
            Err(e) => {
                app.goto_screen(Screen::Error(e));
            }
        },
        _ => {}
    };
}
