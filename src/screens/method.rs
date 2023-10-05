use tui::backend::Backend;
use tui::widgets::ListState;
use tui::Frame;

use crate::app::App;
use crate::display::menuopts::{HOME_MENU_PARAGRAPH, METHOD_MENU_OPTIONS, METHOD_MENU_TITLE};
use crate::request::command::Command;
use crate::screens::screen::Screen;
use crate::ui::default_rect;
use crate::ui::render::render_header_paragraph;

pub fn handle_method_select_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // Added this command init here because it was causing a panic because it wasnt initialized.
    app.command = Some(Command::default());

    let area = default_rect(frame.size());
    let new_list = app.current_screen.get_list();
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.items = app.current_screen.get_opts();
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(new_list, area, &mut state);
    frame.render_widget(
        render_header_paragraph(&HOME_MENU_PARAGRAPH, &METHOD_MENU_TITLE),
        frame.size(),
    );
    match app.selected {
        Some(num) => {
            app.command
                .as_mut()
                .unwrap()
                .set_method(String::from(METHOD_MENU_OPTIONS[num])); // safe index
            app.goto_screen(Screen::RequestMenu(String::from(
                METHOD_MENU_OPTIONS[num].clone(),
            )));
        }
        None => {}
    }
}
