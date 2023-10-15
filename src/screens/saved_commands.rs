use super::render::handle_screen_defaults;
use super::Screen;
use crate::app::App;
use tui::backend::Backend;
use tui::prelude::{Constraint, Direction, Layout, Margin};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem, ListState};
use tui::Frame;

pub fn handle_saved_commands_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    handle_screen_defaults(app, frame);
    match app.selected {
        Some(cmd) => {
            app.goto_screen(Screen::AlertMenu(cmd));
            return;
        }
        _ => {}
    }
}

pub fn handle_alert_menu<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, cmd: usize) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(13),     // This will be the alert box
                Constraint::Percentage(65), // This aligns the main screen perfectly with the bottom
                Constraint::Percentage(22),
            ]
            .as_ref(),
        )
        .split(frame.size());
    // Render the alert box
    let alert_box = layout[0];
    let alert_text_chunk = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Red).fg(Color::White))
        .title("Alert");
    // Render the options
    let options = vec!["Execute", "Copy to Clipboard", "Cancel"];
    let options_box = layout[0].inner(&Margin {
        vertical: 1,
        horizontal: 1,
    });
    let mut list_state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.state = Some(list_state.clone());
    let items: Vec<ListItem> = options
        .iter()
        .map(|option| ListItem::new(*option))
        .collect();
    let list = List::new(items.clone())
        .block(Block::default())
        .highlight_style(
            Style::default()
                .bg(Color::LightBlue)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    app.items = items;
    app.state = Some(list_state.clone());
    app.selected = None;
    app.cursor = 0;
    frame.render_widget(alert_text_chunk, alert_box);
    frame.render_stateful_widget(list, options_box, &mut list_state);
    match app.selected {
        // execute saved command
        Some(0) => {
            app.execute_saved_command(cmd);
            app.goto_screen(Screen::Response(app.response.clone().unwrap()));
        }
        // copy to clipboard
        Some(1) => {
            if let Err(e) = app.copy_cmd_to_clipboard(cmd) {
                app.goto_screen(Screen::Error(e.to_string()));
            }
            app.goto_screen(Screen::Success);
        }
        Some(2) => {
            app.goto_screen(Screen::SavedCommands);
        }
        _ => {}
    }
}
