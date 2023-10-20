use super::{default_rect, small_alert_box};
use crate::app::App;
use crate::display::inputopt::InputOpt;

use crate::request::response::Response;
use crate::screens::screen::Screen;
use std::error::Error;
use tui::backend::Backend;
use tui::text::Text;
use tui::widgets::{ListState, Paragraph};
use tui::Frame;

pub fn handle_response_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, resp: String) {
    let area = default_rect(small_alert_box(frame.size()));
    let new_list = app.current_screen.get_list(None);
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    if !app.items.is_empty() {
        app.items.clear();
    }
    app.items = app.current_screen.get_opts(None);
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(new_list, area, &mut state);
    if let Some(num) = app.selected {
        match num {
            0 => {
                app.goto_screen(Screen::InputMenu(InputOpt::Execute));
            }
            // View response headers
            1 => {
                let area_2 = small_alert_box(frame.size());
                let response = Response::from_raw_string(resp.as_str()).unwrap();
                let headers = response.get_headers();
                let paragraph = Paragraph::new(Text::from(headers.to_string()));
                frame.render_widget(paragraph, area_2);
                app.goto_screen(Screen::SavedCommands);
            }
            // View response body
            2 => {
                app.goto_screen(Screen::ViewBody);
            }
            // Copy to clipboard
            3 => {
                if app.command.is_some() {
                    match terminal_clipboard::set_string(
                        app.command.as_mut().unwrap().get_command_string().as_str(),
                    ) {
                        Ok(_) => app.goto_screen(Screen::Success),
                        Err(e) => app.goto_screen(Screen::Error(e.to_string())),
                    }
                } else if terminal_clipboard::set_string(
                    app.response
                        .as_ref()
                        .unwrap_or(&"Command failed to save".to_string()),
                )
                .is_ok()
                {
                    app.goto_screen(Screen::Success);
                } else {
                    app.goto_screen(Screen::Error("Failed to copy to clipboard".to_string()));
                }
            }
            _ => {}
        };
    }
}
