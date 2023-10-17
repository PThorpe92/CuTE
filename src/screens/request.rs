use super::render::handle_screen_defaults;
use crate::app::App;
use crate::display::inputopt::InputOpt;
use crate::display::AppOptions;
use crate::request::command::CmdType;
use crate::screens::error_alert_box;
use crate::screens::screen::Screen;
use tui::backend::Backend;
use tui::Frame;

pub fn handle_request_menu_screen<B: Backend>(
    app: &mut App,
    frame: &mut Frame<'_, B>,
    err: String,
) {
    handle_screen_defaults(app, frame);
    if !err.is_empty() {
        error_alert_box(frame, &err);
    }
    match app.selected {
        // Add file to upload
        Some(0) => app.goto_screen(Screen::InputMenu(InputOpt::UploadFile)),
        // Add a URL,
        Some(1) => app.goto_screen(Screen::InputMenu(InputOpt::URL(CmdType::Curl))),
        // Add Unix Socket address
        Some(2) => app.goto_screen(Screen::InputMenu(InputOpt::UnixSocket)),
        // Auth
        Some(3) => app.goto_screen(Screen::Authentication),
        // Headers
        Some(4) => app.goto_screen(Screen::InputMenu(InputOpt::Headers)),
        // Verbose
        Some(5) => app.add_app_option(AppOptions::Verbose),
        // Enable headers in response
        Some(6) => app.add_app_option(AppOptions::EnableHeaders),
        // Request Body
        Some(7) => app.goto_screen(Screen::InputMenu(InputOpt::RequestBody)),
        // Save this command
        Some(8) => app.add_app_option(AppOptions::SaveCommand),
        // Save your token or login
        Some(9) => app.add_app_option(AppOptions::SaveToken),
        // Execute command
        Some(10) => match app.execute_command() {
            Ok(()) => {
                let response = app.command.as_mut().unwrap().get_response();
                app.response = Some(response.clone());
                app.goto_screen(Screen::Response(response));
            }
            Err(e) => {
                app.goto_screen(Screen::Error(e.to_string()));
            }
        },
        // more options
        Some(11) => app.goto_screen(Screen::MoreFlags),
        // clear options
        Some(12) => app.remove_all_app_options(),
        _ => {}
    }
}
