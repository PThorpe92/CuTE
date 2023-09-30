use tui::backend::Backend;
use tui::widgets::ListState;
use tui::Frame;

use crate::app::App;
use crate::screens::screen::Screen;
use crate::ui::widgets::boxes::centered_rect;
use crate::ui::widgets::menu::menu_widget;

pub fn handle_debug_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let menu_options = app.current_screen.get_list();
    let area = centered_rect(70, 60, frame.size());

    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));

    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));

    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(menu_options, area, &mut state);
    frame.render_widget(menu_widget(), frame.size());

    match app.selected {
        Some(0) => {
            // Back To Home
            app.goto_screen(Screen::Home); // Back
        }
        Some(1) => {
            // Test Single Line Input Screen
            app.goto_screen(Screen::TestInput(app.input.to_string()));
        }
        _ => {}
    }
}
