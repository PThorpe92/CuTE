use super::render::handle_screen_defaults;
use crate::app::App;
use crate::request::command::Command;
use crate::request::curl::Curl;
use crate::request::wget::Wget;
use crate::screens::screen::Screen;
use tui::backend::Backend;
use tui::Frame;

pub fn handle_home_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    handle_screen_defaults(app, frame);
    if let Some(num) = app.selected {
        match num {
            0 => {
                app.set_command(Command::Curl(Curl::new()));
                app.goto_screen(Screen::Method);
            }
            1 => {
                app.set_command(Command::Wget(Wget::new()));
                app.goto_screen(Screen::Downloads);
            }
            2 => app.goto_screen(Screen::SavedKeys),
            3 => app.goto_screen(Screen::SavedCommands),
            _ => {}
        }
    }
}
