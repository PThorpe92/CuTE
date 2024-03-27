use super::Screen;
use crate::app::App;
use crate::display::inputopt::InputOpt;
use crate::display::menuopts::{
    COLLECTION_ALERT_MENU_OPTS, DEFAULT_MENU_PARAGRAPH, POSTMAN_COLLECTION_TITLE,
};
use crate::screens::render::handle_screen_defaults;
use crate::screens::{centered_rect, render::render_header_paragraph};
use tui::prelude::{Constraint, Direction, Layout, Margin};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use tui::Frame;

pub fn handle_collection_menu(app: &mut App, frame: &mut Frame<'_>) {
    handle_screen_defaults(app, frame);
    match app.selected {
        // Import New Collection
        Some(0) => app.goto_screen(&Screen::InputMenu(InputOpt::ImportCollection)),
        // Create New Collection
        Some(1) => app.goto_screen(&Screen::InputMenu(InputOpt::CreateCollection)),
        // View Saved Collections
        Some(2) => app.goto_screen(&Screen::ViewSavedCollections),
        // Cancel
        Some(3) => {
            app.goto_screen(&Screen::Home);
        }
        _ => {}
    }
}

pub fn handle_collections_screen(app: &mut App, frame: &mut Frame<'_>) {
    let collections = app.get_collections().unwrap_or_default();
    let items = Some(
        collections
            .into_iter()
            .map(|x| x.name)
            .collect::<Vec<String>>(),
    );
    let menu_options = app.current_screen.get_list(items);
    let area = centered_rect(70, 60, frame.size());
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(menu_options, area, &mut state);
    let (paragraph, title) = (&DEFAULT_MENU_PARAGRAPH, &POSTMAN_COLLECTION_TITLE);
    frame.render_widget(
        render_header_paragraph(paragraph, title, app.config.get_style()),
        frame.size(),
    );
    if let Some(selected) = app.selected {
        app.goto_screen(&Screen::ColMenu(selected));
    }
}

pub fn handle_collection_alert_menu(app: &mut App, frame: &mut Frame<'_>, cmd: usize) {
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
    let items: Vec<ListItem> = COLLECTION_ALERT_MENU_OPTS
        .iter()
        .map(|option| ListItem::new(*option))
        .collect();
    let list = List::new(items.clone())
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
    let show_cmds = app.get_collections().unwrap_or_default();
    if let Some(selected) = show_cmds.get(cmd) {
        let paragraph = Paragraph::new(format!("{:?}", selected.name))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Request Collection"),
            )
            .alignment(tui::layout::Alignment::Center);
        frame.render_widget(paragraph, cmd_str);
        frame.render_widget(alert_text_chunk, alert_box);
        frame.render_stateful_widget(list, options_box, &mut list_state);
        match app.selected {
            // View Requests in collection
            Some(0) => {
                app.goto_screen(&Screen::SavedCommands(Some(selected.id)));
            }
            // Rename Collection
            Some(1) => app.goto_screen(&Screen::InputMenu(InputOpt::RenameCollection(selected.id))),
            // delete collection
            Some(2) => {
                if let Err(e) = app.delete_collection(selected.id) {
                    app.goto_screen(&Screen::Error(e.to_string()));
                }
                app.goto_screen(&Screen::Success);
            }
            // cancel
            Some(3) => {
                app.goto_screen(&Screen::ViewSavedCollections);
            }
            _ => {}
        }
    }
}
