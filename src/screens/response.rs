use super::{default_rect, small_alert_box};
use crate::app::App;
use crate::database::db::DB;
use crate::display::inputopt::InputOpt;
use crate::display::DisplayOpts;
use crate::request::command::Command;
use crate::screens::screen::Screen;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::error::Error;
use tui::backend::Backend;
use tui::layout::Alignment;
use tui::style::{Color, Style};
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
    let new_list = app.current_screen.get_list();
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    let paragraph = Paragraph::new(Text::from(resp.as_str()))
        .style(Style::default().fg(Color::Yellow).bg(Color::Black))
        .alignment(Alignment::Center);
    if !app.items.is_empty() {
        app.items.clear();
    }
    // now we can check for whether or not we needed to store the key
    if app.has_display_option(&DisplayOpts::SaveToken) {
        if app.db.is_none() {
            app.db = Some(Box::new(DB::new().unwrap()));
        }
        app.command
            .as_ref()
            .unwrap()
            .save_key(app.db.as_ref().unwrap());
    }
    app.items = app.current_screen.get_opts();
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(new_list, area, &mut state);
    let area_2 = small_alert_box(frame.size());
    frame.render_widget(paragraph, area_2);
    match app.selected {
        Some(num) => match num {
            0 => {
                app.goto_screen(Screen::InputMenu(InputOpt::Execute));
            }
            // View response headers
            1 => {
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
