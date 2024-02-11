use std::fmt::{Display, Formatter};


use tui::Frame;

use super::render::handle_screen_defaults;
use crate::app::App;
use crate::display::inputopt::InputOpt;
use crate::display::menuopts::{AWS_AUTH_ERROR_MSG, AWS_AUTH_MSG};
use crate::display::AppOptions;
use crate::request::curl::AuthKind;
use crate::screens::screen::Screen;

// This is the display auth not to be confused with the request auth
// it needs to be done away with and combined into one
#[derive(Debug, Clone, PartialEq)]
pub enum AuthType {
    Basic,
    Bearer,
    Digest,
    AWSSignatureV4,
    NTLM,
    SPNEGO,
}

#[rustfmt::skip]
impl Display for AuthType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let auth = match self {
            AuthType::Basic          => "Basic",
            AuthType::Bearer         => "Bearer",
            AuthType::Digest         => "Digest",
            AuthType::AWSSignatureV4 => "AWS Signature V4",
            AuthType::NTLM           => "NTLM",
            AuthType::SPNEGO         => "SPNEGO",
        };
        write!(f, "{}", auth)
    }
}
pub fn handle_authentication_screen(app: &mut App, frame: &mut Frame<'_>) {
    handle_screen_defaults(app, frame);
    if let Some(num) = app.selected {
        match num {
            0 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthType::Basic))),
            1 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthType::Bearer))),
            2 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthType::Digest))),
            3 => {
                if varify_aws_auth() {
                    app.goto_screen(Screen::RequestMenu(String::from(AWS_AUTH_MSG)));
                    app.add_app_option(AppOptions::Auth(AuthKind::AwsSigv4));
                } else {
                    app.goto_screen(Screen::RequestMenu(String::from(AWS_AUTH_ERROR_MSG)));
                }
            }
            4 => {
                app.add_app_option(AppOptions::Auth(AuthKind::Spnego));
                app.goto_screen(Screen::RequestMenu(String::from("")));
            }
            5 => {
                app.add_app_option(AppOptions::Auth(AuthKind::Ntlm));
                app.goto_screen(Screen::RequestMenu(String::from(
                    "Alert: NTLM Auth Enabled",
                )));
            }
            _ => {}
        }
    }
}

fn varify_aws_auth() -> bool {
    if std::env::var("AWS_ACCESS_KEY_ID").is_ok()
        && std::env::var("AWS_SECRET_ACCESS_KEY").is_ok()
        && std::env::var("AWS_DEFAULT_REGION").is_ok()
    {
        return true;
    }
    false
}
