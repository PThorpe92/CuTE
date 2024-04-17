use super::{error_alert_box, Screen};
use crate::app::App;
use crate::display::inputopt::InputOpt;
use crate::display::menuopts::{
    COLLECTION_ALERT_MENU_OPTS, DEFAULT_MENU_PARAGRAPH, POSTMAN_COLLECTION_TITLE,
};
use crate::screens::render::handle_screen_defaults;
use crate::screens::{
    centered_rect, input::input_screen::handle_default_input_screen,
    render::render_header_paragraph, ScreenArea,
};
use tui::prelude::{Constraint, Direction, Layout, Margin};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use tui::Frame;

pub fn handle_collection_menu(app: &mut App, frame: &mut Frame<'_>, opt: Option<InputOpt>) {
    handle_screen_defaults(app, frame);
    match opt {
        Some(InputOpt::RequestError(e)) => {
            error_alert_box(frame, &e);
        }
        Some(opt) => {
            handle_default_input_screen(app, frame, opt.clone());
        }
        _ => {}
    };
    match app.selected {
        // Import New Collection
        Some(0) => app.goto_screen(&Screen::SavedCollections(Some(InputOpt::ImportCollection))),
        // View Saved Collections
        Some(1) => app.goto_screen(&Screen::ViewSavedCollections),
        // Cancel
        Some(2) => {
            app.goto_screen(&Screen::Home);
        }
        _ => {}
    }
}

pub fn handle_collections_screen(app: &mut App, frame: &mut Frame<'_>) {
    let collections = app.db.as_ref().get_collections().unwrap_or_default();
    let items = Some(
        collections
            .clone()
            .into_iter()
            .map(|x| x.get_name().to_string())
            .collect::<Vec<String>>(),
    );
    let menu_options = app.current_screen.get_list(items);
    let area = centered_rect(frame.size(), ScreenArea::Center);
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
        let selected = collections.get(selected).unwrap();
        app.goto_screen(&Screen::ColMenu(selected.get_id()));
    }
}

pub fn handle_collection_alert_menu(app: &mut App, frame: &mut Frame<'_>, cmd: i32) {
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
        .title("Collection Menu");
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
    let selected = app
        .db
        .as_ref()
        .get_collection_by_id(cmd)
        .unwrap_or_default();
    let count = app
        .db
        .get_number_of_commands_in_collection(cmd)
        .unwrap_or_default();
    let paragraph = Paragraph::new(format!(
        "{:?}\nContains: {} {}",
        selected.get_name(),
        count,
        if count == 1 { "request" } else { "requests" }
    ))
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
        Some(0) => app.goto_screen(&Screen::SavedCommands {
            id: Some(cmd),
            opt: None,
        }),
        // Rename Collection
        Some(1) => app.goto_screen(&Screen::SavedCollections(Some(InputOpt::RenameCollection(
            selected.get_id(),
        )))),
        // delete collection
        Some(2) => {
            if let Err(e) = app.db.as_ref().delete_collection(selected.get_id()) {
                app.goto_screen(&Screen::SavedCollections(Some(InputOpt::RequestError(
                    format!("Error: {e}"),
                ))));
            }
            app.goto_screen(&Screen::SavedCollections(Some(InputOpt::AlertMessage(
                String::from("Success: collection deleted"),
            ))));
        }
        // cancel
        Some(3) => {
            app.goto_screen(&Screen::ViewSavedCollections);
        }
        _ => {}
    }
}
