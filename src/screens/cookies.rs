use tui::Frame;

use crate::app::App;
use crate::display::inputopt::InputOpt;
use crate::display::AppOptions;

use super::render::handle_screen_defaults;
use super::Screen;

pub fn handle_cookies_menu(app: &mut App, frame: &mut Frame<'_>) {
    handle_screen_defaults(app, frame);
    if let Some(num) = app.selected {
        match num {
            // set cookie filepath
            0 => app.goto_screen(&Screen::RequestMenu(Some(InputOpt::CookiePath))),
            // set cookie jar path
            1 => app.goto_screen(&Screen::RequestMenu(Some(InputOpt::CookieJar))),
            // new cookie
            2 => app.goto_screen(&Screen::RequestMenu(Some(InputOpt::NewCookie))),
            3 => {
                app.add_app_option(AppOptions::NewCookieSession);
                app.goto_screen(&Screen::RequestMenu(None));
            }
            4 => app.goto_screen(&Screen::RequestMenu(None)),
            _ => {}
        }
    }
}
