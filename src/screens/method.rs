use tui::backend::Backend;
use tui::widgets::ListState;
use tui::Frame;

use crate::app::App;
use crate::display::menuopts::METHOD_MENU_OPTIONS;
use crate::request::command::Command;
use crate::request::curl::Curl;
use crate::screens::screen::Screen;
use crate::ui::widgets::boxes::default_rect;
use crate::ui::widgets::menu::menu_widget;

pub fn handle_method_select_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let area = default_rect(frame.size());
    let new_list = app.current_screen.get_list();
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.items = app.current_screen.get_opts();
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(new_list, area, &mut state);
    frame.render_widget(menu_widget(), frame.size());
    app.command = Some(Command::Curl(Curl::new()));
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
