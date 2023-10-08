use super::render::handle_screen_defaults;
use super::screen::determine_line_size;
use super::Screen;
use crate::app::App;
use tui::backend::Backend;
use tui::style::{Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem};
use tui::Frame;

pub fn handle_saved_commands_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    handle_screen_defaults(app, frame);
    // This means they have pressed Enter and want to execute the command
    let alert_box = super::small_rect(frame.size());
    let prompt = vec![ListItem::new(format!(
        "Execute command?{}",
        determine_line_size()
    ))];
    let list = List::new(prompt)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(list, alert_box, &mut app.state.as_mut().unwrap());
    match app.selected {
        Some(cmd) => {
            app.execute_saved_command(cmd);
            app.goto_screen(Screen::Response(app.response.as_ref().unwrap().clone()));
            app.selected = None;
        }
        None => {}
    }
}
