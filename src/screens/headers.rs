use tui::Frame;

use super::render::handle_screen_defaults;
use super::Screen;
use crate::app::App;
use crate::display::inputopt::InputOpt;
use crate::display::{AppOptions, HeaderKind};

pub fn handle_headers_screen(app: &mut App, frame: &mut Frame<'_>) {
    handle_screen_defaults(app, frame);

    match app.selected {
        // add custom headers
        // "Add Content-Type: application/json  ",
        // "Add Content-Type: application/xml  ",
        // "Add Content-Type: application/X-WWW-Form-Urlencoded  ",
        // "Add Accept: application/json  ",
        // "Add Accept: text/html  ",
        // "Add Accept: application/xml  ",
        Some(0) => app.goto_screen(&Screen::RequestMenu(Some(InputOpt::Headers))),
        // add content-type application/json
        Some(1) => {
            app.add_app_option(AppOptions::ContentHeaders(HeaderKind::ContentType(
                String::from("application/json"),
            )));
        }
        // add accept application/json
        Some(2) => {
            app.add_app_option(AppOptions::ContentHeaders(HeaderKind::ContentType(
                String::from("application/xml"),
            )));
        }
        Some(3) => {
            app.add_app_option(AppOptions::ContentHeaders(HeaderKind::ContentType(
                String::from("application/www-form-urlencoded"),
            )));
        }
        // add accept application/json
        Some(4) => {
            app.add_app_option(AppOptions::ContentHeaders(HeaderKind::Accept(
                String::from("application/json"),
            )));
        }
        Some(5) => {
            app.add_app_option(AppOptions::ContentHeaders(HeaderKind::ContentType(
                String::from("text/html"),
            )));
        }
        // add accept application/json
        Some(6) => {
            app.add_app_option(AppOptions::ContentHeaders(HeaderKind::Accept(
                String::from("application/xml"),
            )));
        }
        // accept headers in response
        Some(7) => {
            app.add_app_option(AppOptions::EnableHeaders);
        }
        // return to request menu
        Some(8) => app.goto_screen(&Screen::RequestMenu(None)),
        _ => {}
    }
}
