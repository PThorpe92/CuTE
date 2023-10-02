use crate::app::App;
use crate::ui::render::render_header_paragraph;
use crate::ui::widgets::boxes::default_rect;
use tui::backend::Backend;

use tui::widgets::{ListState};
use tui::Frame;

pub fn handle_api_key_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let area = default_rect(frame.size());
    let new_list = app.current_screen.get_list();
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    if !app.items.is_empty() {
        app.items.clear();
    }
    app.items = app.current_screen.get_opts();
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));

    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(new_list, area, &mut state);
    frame.render_widget(
        render_header_paragraph(API_KEY_PARAGRAPH, API_KEY_TITLE),
        frame.size(),
    );
}

const API_KEY_PARAGRAPH: &'static str = "Create / Edit / Delete API Keys and tokens.\n      Press q to exit      \n Press Enter to select \n Please select a Menu item\n";
const API_KEY_TITLE: &'static str = "My API Keys";
