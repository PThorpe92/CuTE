use super::render::handle_screen_defaults;
use crate::app::App;
use crate::display::inputopt::InputOpt;
use crate::display::menuopts::{AWS_AUTH_ERROR_MSG, AWS_AUTH_MSG};
use crate::display::AppOptions;
use crate::request::curl::AuthKind;
use crate::screens::screen::Screen;
use tui::Frame;

pub fn handle_authentication_screen(app: &mut App, frame: &mut Frame<'_>) {
    handle_screen_defaults(app, frame);
    if let Some(num) = app.selected {
        match num {
            0 => app.goto_screen(&Screen::RequestMenu(Some(InputOpt::Auth(AuthKind::Basic(
                "".to_string(),
            ))))),
            1 => app.goto_screen(&Screen::RequestMenu(Some(InputOpt::Auth(
                AuthKind::Bearer("".to_string()),
            )))),
            2 => app.goto_screen(&Screen::RequestMenu(Some(InputOpt::Auth(
                AuthKind::Digest("".to_string()),
            )))),
            3 => {
                if varify_aws_auth() {
                    app.goto_screen(&Screen::RequestMenu(Some(InputOpt::RequestError(
                        String::from(AWS_AUTH_MSG),
                    ))));
                    app.add_app_option(AppOptions::Auth(AuthKind::AwsSigv4));
                } else {
                    app.goto_screen(&Screen::RequestMenu(Some(InputOpt::RequestError(
                        String::from(AWS_AUTH_ERROR_MSG),
                    ))));
                }
            }
            4 => {
                if app.command.has_auth() {
                    app.remove_app_option(&AppOptions::Auth(AuthKind::None));
                }
                app.add_app_option(AppOptions::Auth(AuthKind::Spnego));
                app.goto_screen(&Screen::RequestMenu(None));
            }
            5 => {
                if app.command.has_auth() {
                    app.remove_app_option(&AppOptions::Auth(AuthKind::None));
                }
                app.add_app_option(AppOptions::Auth(AuthKind::Ntlm));
                app.goto_screen(&Screen::RequestMenu(Some(InputOpt::RequestError(
                    String::from("Alert: NTLM Auth Enabled"),
                ))));
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
