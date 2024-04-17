use super::render::render_header_paragraph;
use super::{centered_rect, Screen, ScreenArea};
use crate::app::App;
use crate::display::menuopts::{CMD_MENU_OPTIONS, SAVED_COMMANDS_PARAGRAPH, SAVED_COMMANDS_TITLE};
use tui::prelude::{Constraint, Direction, Layout, Margin};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use tui::Frame;

pub fn handle_saved_commands_screen(app: &mut App, frame: &mut Frame<'_>, coll: Option<i32>) {
    let commands = app.db.as_ref().get_commands(coll).unwrap_or_default();
    let items = Some(
        commands
            .iter()
            .map(|x| {
                format!(
                    "Request: {} Collection: {:?}",
                    x.get_command(),
                    x.get_collection_id()
                )
            })
            .collect::<Vec<String>>(),
    );
    let menu_options = app.current_screen.get_list(items);
    let area = centered_rect(frame.size(), ScreenArea::Center);

    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.render_stateful_widget(menu_options, area, &mut state);
    let (paragraph, title) = (&SAVED_COMMANDS_PARAGRAPH, &SAVED_COMMANDS_TITLE);
    frame.render_widget(
        render_header_paragraph(paragraph, title, app.config.get_style()),
        frame.size(),
    );
    if let Some(selected) = app.selected {
        let cmd = commands.get(selected);
        if let Some(cmd) = cmd {
            app.goto_screen(&Screen::CmdMenu(cmd.get_id()));
        } else {
            app.goto_screen(&Screen::Error("No commands found".to_string()));
        }
    }
}

pub fn handle_alert_menu(app: &mut App, frame: &mut Frame<'_>, cmd: i32) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .horizontal_margin(5)
        .split(frame.size());
    // Render the alert box
    let alert_box = layout[1];
    let alert_text_chunk = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black).fg(Color::LightGreen))
        .title("Alert");
    let options_box = layout[1].inner(&Margin {
        vertical: 1,
        horizontal: 1,
    });
    let mut list_state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.state = Some(list_state.clone());
    let items: Vec<ListItem> = CMD_MENU_OPTIONS
        .iter()
        .map(|option| ListItem::new(*option))
        .collect();
    let list = List::new(items)
        .block(Block::default())
        .highlight_style(
            Style::default()
                .bg(Color::White)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    let cmd_str = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(alert_box)[1];
    if let Ok(command) = app.db.as_ref().get_command_by_id(cmd) {
        let paragraph = Paragraph::new(format!("{:?}", command))
            .block(Block::default().borders(Borders::ALL).title("Command"))
            .alignment(tui::layout::Alignment::Center);
        frame.render_widget(paragraph, cmd_str);
        frame.render_widget(alert_text_chunk, alert_box);
        frame.render_stateful_widget(list, options_box, &mut list_state);
        match app.selected {
            // execute saved command
            Some(0) => {
                app.execute_saved_command(command.get_curl_json());
                app.goto_screen(&Screen::Response(app.response.clone().unwrap()));
            }
            // delete item
            Some(1) => {
                if let Err(e) = app.delete_item(command.get_id()) {
                    app.goto_screen(&Screen::Error(e.to_string()));
                } else {
                    app.goto_screen(&Screen::Success);
                }
            }
            // copy to clipboard
            Some(2) => {
                if let Err(e) = app.copy_to_clipboard(command.get_command()) {
                    app.goto_screen(&Screen::Error(e.to_string()));
                }
                app.goto_screen(&Screen::Success);
            }
            // cancel
            Some(3) => {
                app.goto_screen(&Screen::SavedCommands(None));
            }
            _ => {}
        }
    }
}
