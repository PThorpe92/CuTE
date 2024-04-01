use crate::app::App;
use crate::display::inputopt::InputOpt;
use crate::request::response::Response;
use crate::screens::{centered_rect, screen::Screen, ScreenArea};
use tui::text::Text;
use tui::widgets::{ListState, Paragraph};
use tui::Frame;

pub fn handle_response_screen(app: &mut App, frame: &mut Frame<'_>, resp: String) {
    let area = centered_rect(frame.size(), ScreenArea::Center);
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
                app.goto_screen(&Screen::InputMenu(InputOpt::Execute));
            }
            // View response headers
            1 => {
                let area_2 = centered_rect(frame.size(), ScreenArea::Center);
                // Check for response error here
                let response = match Response::from_raw_string(resp.as_str()) {
                    Ok(resp) => resp,
                    Err(e) => {
                        // Hit the error screen.
                        app.goto_screen(&Screen::Error(String::from(e)));
                        return;
                    }
                };
                let headers = response.get_headers();
                let paragraph = Paragraph::new(Text::from(headers.to_string()));
                frame.render_widget(paragraph, area_2);
                //app.goto_screen(&Screen::SavedCommands);
            }
            // View response body
            2 => {
                app.goto_screen(&Screen::ViewBody);
            }
            // Copy to clipboard
            3 => {
                let cmd_str = app.command.get_command_string();
                app.copy_to_clipboard(&cmd_str).unwrap_or_else(|e| {
                    app.goto_screen(&Screen::Error(e));
                });
                app.goto_screen(&Screen::Success);
            }
            4 => {
                // Return To Home
                app.clear_all_options();
                app.goto_screen(&Screen::Home);
            }
            _ => {}
        };
    }
}
