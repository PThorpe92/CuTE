use super::default_rect;
use super::render::{handle_screen_defaults, render_header_paragraph};
use crate::app::App;
use crate::display::inputopt::InputOpt;
use crate::display::menuopts::{API_KEY_PARAGRAPH, API_KEY_TITLE};
use tui::backend::Backend;

use tui::widgets::{List, ListItem, ListState};
use tui::Frame;

use super::Screen;

pub fn handle_api_key_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    handle_screen_defaults(app, frame);
    match app.selected {
        Some(0) => {
            app.goto_screen(Screen::InputMenu(InputOpt::ApiKey));
            app.selected = None;
        }
        Some(1) => app.goto_screen(Screen::SavedKeys),
        Some(2) => app.goto_screen(Screen::SavedKeys),
        _ => {}
    }
}

pub fn handle_view_saved_keys<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let area = default_rect(frame.size());
    let saved_keys = app.get_saved_keys().unwrap_or(vec![]);
    let new_list = match saved_keys.len() {
        0 => vec![ListItem::new("No Saved Commands")],
        _ => saved_keys
            .iter()
            .map(|c| ListItem::new(c.clone()))
            .collect(),
    };
    let new_list = List::new(new_list);
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.items = app.current_screen.get_opts();
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));

    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(new_list, area, &mut state);
    frame.render_widget(
        render_header_paragraph(&API_KEY_PARAGRAPH, &API_KEY_TITLE),
        frame.size(),
    );
}
