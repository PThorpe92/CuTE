use tui::widgets::ListState;
use tui::Frame;

use crate::app::App;
use crate::screens::screen::Screen;
use crate::ui::widgets::{centered_rect, menu_paragraph};

pub fn handle_home_screen<B>(app: &mut App, frame: &mut Frame<'_, B>) {
    let new_list = app.current_screen.get_list();
    let area = centered_rect(70, 60, frame.size());
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(new_list, area, &mut state);
    frame.render_widget(menu_paragraph(), frame.size());
    match app.selected {
        Some(0) => {
            app.goto_screen(Screen::Method);
        }
        Some(1) => {
            app.goto_screen(Screen::Downloads);
        }
        Some(2) => {
            app.goto_screen(Screen::Keys);
        }
        Some(3) => {
            app.goto_screen(Screen::Commands);
        }
        Some(_) => {}
        None => {}
    }
}
