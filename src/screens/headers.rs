
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
        Some(0) => app.goto_screen(&Screen::InputMenu(InputOpt::Headers)),
        //
        // add content-type application/json
        Some(1) => {
            app.add_app_option(AppOptions::ContentHeaders(HeaderKind::ContentType));
        }
        //
        // add accept application/json
        Some(2) => {
            app.add_app_option(AppOptions::ContentHeaders(HeaderKind::Accept));
        }
        //
        // accept headers in response
        Some(3) => {
            app.add_app_option(AppOptions::EnableHeaders);
        }
        //
        // return to request menu
        Some(4) => app.goto_screen(&Screen::RequestMenu(String::new())),
        _ => {}
    }
}
