use super::render::handle_screen_defaults;
use crate::app::App;
use crate::display::menuopts::METHOD_MENU_OPTIONS;
use crate::request::command::Cmd;
use crate::request::curl::Curl;
use crate::screens::screen::Screen;
use tui::backend::Backend;
use tui::Frame;

pub fn handle_method_select_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    app.command = Some(Box::new(Cmd::Curl(Curl::new())));
    handle_screen_defaults(app, frame);
    if let Some(num) = app.selected {
        app.command
            .as_mut()
            .unwrap()
            .set_method(String::from(METHOD_MENU_OPTIONS[num])); // safe index
        app.goto_screen(Screen::RequestMenu(String::from(METHOD_MENU_OPTIONS[num])));
    }
}
