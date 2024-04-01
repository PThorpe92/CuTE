use super::render::handle_screen_defaults;
use crate::app::App;
use crate::display::menuopts::METHOD_MENU_OPTIONS;
use crate::request::curl::{Curl, Method};
use crate::screens::screen::Screen;
use std::str::FromStr;
use tui::Frame;

pub fn handle_method_select_screen(app: &mut App, frame: &mut Frame<'_>) {
    app.clear_all_options();
    app.set_command(Curl::new());
    handle_screen_defaults(app, frame);
    if let Some(num) = app.selected {
        app.command
            .set_method(Method::from_str(METHOD_MENU_OPTIONS[num]).unwrap_or(Method::Get)); // safe index
        app.goto_screen(&Screen::RequestMenu(None));
    }
}
