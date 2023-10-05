use tui::backend::Backend;
use tui::Frame;

use crate::app::App;
use crate::display::menuopts::METHOD_MENU_OPTIONS;
use crate::request::command::Command;
use crate::screens::screen::Screen;

use super::render::handle_screen_defaults;

pub fn handle_method_select_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    if app.command.is_none() {
        app.set_command(Command::default());
    }
    handle_screen_defaults(app, frame);
    match app.selected {
        Some(num) => {
            app.command
                .as_mut()
                .unwrap()
                .set_method(String::from(METHOD_MENU_OPTIONS[num])); // safe index
            app.goto_screen(Screen::RequestMenu(String::from(
                METHOD_MENU_OPTIONS[num].clone(),
            )));
        }
        None => {}
    }
}
