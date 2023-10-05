use std::fmt::{Display, Formatter};

use tui::backend::Backend;
use tui::Frame;

use super::render::handle_screen_defaults;
use crate::app::App;
use crate::display::inputopt::InputOpt;
use crate::screens::screen::Screen;

// This is the display auth not to be confused with the request auth

#[derive(Debug, Clone, PartialEq)]
pub enum AuthType {
    // OAuth looks impossible to implement
    Basic,
    Bearer,
    Digest,
    AWSSignatureV4,
    NTLM,
    NTLMWB,
    Kerberos,
    SPNEGO,
}

impl Display for AuthType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let auth = match self {
            AuthType::Basic => "Basic",
            AuthType::Bearer => "Bearer",
            AuthType::Digest => "Digest",
            AuthType::AWSSignatureV4 => "AWS Signature V4",
            AuthType::NTLM => "NTLM",
            AuthType::NTLMWB => "NTLMWB",
            AuthType::Kerberos => "Kerberos",
            AuthType::SPNEGO => "SPNEGO",
        };
        write!(f, "{}", auth)
    }
}
pub fn handle_authentication_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    handle_screen_defaults(app, frame);
    if let Some(num) = app.selected {
        match num {
            0 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthType::Basic))),
            1 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthType::Bearer))),
            2 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthType::Digest))),
            3 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthType::AWSSignatureV4))),
            4 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthType::SPNEGO))),
            5 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthType::Kerberos))),
            6 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthType::NTLM))),
            _ => {}
        }
    }
}
