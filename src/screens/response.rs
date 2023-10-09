use super::{default_rect, small_alert_box};
use crate::app::App;
use crate::display::inputopt::InputOpt;
use crate::request::command::Command;
use crate::request::response::Response;
use crate::screens::screen::Screen;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::error::Error;
use tui::backend::Backend;
use tui::text::Text;
use tui::widgets::{ListState, Paragraph};
use tui::Frame;

fn copy_to_clipboard(command: &str) -> Result<(), Box<dyn Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(command.to_owned())?;
    Ok(())
}

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
    match app.selected {
        Some(num) => match num {
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
                if let Command::Curl(ref mut curl) = app.command.as_mut().unwrap() {
                    match copy_to_clipboard(curl.get_command_str().as_str()) {
                        Ok(_) => app.goto_screen(Screen::Success),
                        Err(e) => app.goto_screen(Screen::Error(e.to_string())),
                    }
                }
            }
            _ => {}
        },
        None => {}
    }
}
