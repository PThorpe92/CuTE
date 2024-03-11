use super::render::handle_screen_defaults;
use crate::app::App;
use crate::display::menuopts::METHOD_MENU_OPTIONS;
use crate::request::command::Cmd;
use crate::request::command::CMD;
use crate::request::curl::Curl;
use crate::screens::screen::Screen;

use tui::Frame;

pub fn handle_method_select_screen(app: &mut App, frame: &mut Frame<'_>) {
    app.remove_all_app_options();
    app.set_command(Box::new(Cmd::Curl(Curl::new())));
    handle_screen_defaults(app, frame);
    if let Some(num) = app.selected {
        app.command.set_method(METHOD_MENU_OPTIONS[num]); // safe index
        app.goto_screen(Screen::RequestMenu(String::from(METHOD_MENU_OPTIONS[num])));
    }
}
