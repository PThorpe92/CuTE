use super::render::handle_screen_defaults;
use crate::app::App;
use crate::screens::screen::Screen;

use tui::Frame;

pub fn handle_home_screen(app: &mut App, frame: &mut Frame<'_>) {
    handle_screen_defaults(app, frame);

    if let Some(num) = app.selected {
        match num {
            0 => {
                app.goto_screen(Screen::Method);
            }
            1 => app.goto_screen(Screen::SavedCollections),
            2 => app.goto_screen(Screen::SavedKeys),
            3 => app.goto_screen(Screen::SavedCommands(None)),
            _ => {}
        }
    }
}
