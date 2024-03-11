use super::render::handle_screen_defaults;
use crate::app::App;
use crate::request::command::Cmd;
use crate::request::wget::Wget;
use crate::screens::screen::Screen;

use tui::Frame;

pub fn handle_home_screen(app: &mut App, frame: &mut Frame<'_>) {
    handle_screen_defaults(app, frame);

    if let Some(num) = app.selected {
        match num {
            0 => {
                app.goto_screen(Screen::Method);
            }
            1 => {
                app.set_command(Box::new(Cmd::Wget(Wget::new())));
                app.goto_screen(Screen::Downloads("".to_string()));
            }
            2 => app.goto_screen(Screen::SavedKeys),
            3 => app.goto_screen(Screen::SavedCommands),
            _ => {}
        }
    }
}
