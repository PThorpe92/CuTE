use tui::Frame;

use crate::app::App;
use crate::display::inputopt::InputOpt;
use crate::display::AppOptions;
use crate::screens::screen::Screen;

use super::render::handle_screen_defaults;

pub fn handle_more_flags_screen(app: &mut App, frame: &mut Frame<'_>) {
    handle_screen_defaults(app, frame);
    match app.selected {
        // follow redirects
        Some(0) => app.add_app_option(AppOptions::FollowRedirects),
        // specify max redirects
        Some(1) => app.goto_screen(&Screen::InputMenu(InputOpt::MaxRedirects)),
        // proxy tunnel
        Some(3) => app.add_app_option(AppOptions::ProxyTunnel),
        // Send auth to hosts if redirected
        Some(4) => app.add_app_option(AppOptions::UnrestrictedAuth),
        // specify referrer
        Some(5) => app.goto_screen(&Screen::RequestMenu(Some(InputOpt::Referrer))),
        // specify ca-path
        Some(6) => app.goto_screen(&Screen::RequestMenu(Some(InputOpt::CaPath))),
        // Request certificate info
        Some(7) => app.add_app_option(AppOptions::CertInfo),
        // fail on error
        Some(8) => app.add_app_option(AppOptions::FailOnError),
        // wildcard match
        Some(9) => app.add_app_option(AppOptions::MatchWildcard),
        // user agent
        Some(10) => app.goto_screen(&Screen::RequestMenu(Some(InputOpt::UserAgent))),
        // enable tcp keepalive
        Some(11) => app.add_app_option(AppOptions::TcpKeepAlive),
        _ => {}
    }
}
