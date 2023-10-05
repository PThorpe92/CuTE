use tui::backend::Backend;
use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::Paragraph;
use tui::Frame;

use crate::app::App;
use crate::screens::input::input::handle_default_input_screen;
use crate::ui::small_rect;

use super::auth::handle_authentication_screen;
use super::debug::handle_debug_screen;
use super::downloads::handle_downloads_screen;
use super::home::handle_home_screen;
use super::input::url::handle_url_input_screen;
use super::keys::{handle_api_key_screen, handle_view_saved_keys};
use super::method::handle_method_select_screen;
use super::request::handle_request_menu_screen;
use super::response::handle_response_screen;
use super::saved_commands::handle_saved_commands_screen;
use super::success::handle_success_screen;
use super::Screen;

pub fn handle_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, screen: Screen) {
    match screen {
        Screen::Home => handle_home_screen(app, frame),
        Screen::Method => handle_method_select_screen(app, frame),
        Screen::ViewBody => {
            let area = small_rect(frame.size());
            let response = app.response.clone().unwrap();
            let paragraph = Paragraph::new(Text::from(response.as_str()))
                .style(Style::default().fg(Color::Yellow).bg(Color::Black))
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, area);
        }
        Screen::Downloads => handle_downloads_screen(app, frame),
        Screen::RequestMenu(_) => handle_request_menu_screen(app, frame),
        // KEYS SCREEN **********************************************************
        Screen::KeysMenu => {
            handle_api_key_screen(app, frame);
        }
        // AUTHENTICATION SCREEN ************************************************
        Screen::Authentication => {
            handle_authentication_screen(app, frame);
        }
        // SUCESSS SCREEN *******************************************************
        Screen::Success => {
            handle_success_screen(app, frame);
        }
        // INPUT MENU SCREEN ****************************************************
        Screen::InputMenu(opt) => {
            handle_default_input_screen(app, frame, opt.clone());
        }
        // RESPONSE SCREEN ******************************************************
        Screen::Response(resp) => {
            handle_response_screen(app, frame, resp.to_string());
        }
        // DEBUG MENU ***********************************************************
        Screen::Debug => {
            handle_debug_screen(app, frame);
        }
        // URL INPUT SCREEN *****************************************************
        Screen::URLInput => {
            handle_url_input_screen(app, frame);
        }
        Screen::SavedCommands => {
            handle_saved_commands_screen(app, frame);
        }
        Screen::Error(e) => {
            handle_response_screen(app, frame, e.to_string());
        }
        Screen::SavedKeys => {
            handle_view_saved_keys(app, frame);
        }
        _ => {}
    }
}
