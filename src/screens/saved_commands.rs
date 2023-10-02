use tui::backend::Backend;


use tui::widgets::{List, ListItem, ListState};
use tui::Frame;

use crate::app::App;
use crate::ui::render::render_header_paragraph;
use crate::ui::widgets::boxes::default_rect;

pub fn handle_saved_commands_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let saved_commands = app.get_saved_commands().unwrap_or(vec![]);
    let new_list = match saved_commands.len() {
        0 => vec![ListItem::new("No Saved Commands")],
        _ => saved_commands
            .iter()
            .map(|c| ListItem::new(c.clone()))
            .collect(),
    };
    app.items = new_list.clone();
    let new_list = List::new(new_list);
    let area = default_rect(frame.size());
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    if !app.items.is_empty() {
        app.items.clear();
    }
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));

    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(new_list, area, &mut state);
    frame.render_widget(
        render_header_paragraph(SAVED_COMMANDS_PARAGRAPH, SAVED_COMMANDS_TITLE),
        frame.size(),
    );

    match app.selected {
        // for now, use the app.response to display the command
        // as if it's a response in the "show response" screen
        Some(index) => app.set_response(String::from(saved_commands.get(index - 1).unwrap())),
        // the problem, is now we need to figure out how to execute the curl command again
        // with libcurl. so we need to turn the Curl struct into a json that we can parse
        // and turn into a runnable curl command.
        None => {}
    }
}
const SAVED_COMMANDS_PARAGRAPH: &'static str = "View / Delete my saved cURL commands.\nPress q to exit\nPress Enter to select\nPress h to go back\n Please select a Menu item\n";
const SAVED_COMMANDS_TITLE: &'static str = "My Saved cURL Commands";
