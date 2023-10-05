use tui::backend::Backend;
use tui::widgets::ListState;
use tui::Frame;

use crate::app::App;
use crate::display::menuopts::{HOME_MENU_PARAGRAPH, HOME_MENU_TITLE};
use crate::request::command::Command;
use crate::request::curl::Curl;
use crate::request::wget::Wget;
use crate::screens::screen::Screen;
use crate::ui::centered_rect;
use crate::ui::render::render_header_paragraph;

pub fn handle_home_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let new_list = app.current_screen.get_list();
    let area = centered_rect(70, 60, frame.size());
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(new_list, area, &mut state);
    frame.render_widget(
        render_header_paragraph(&HOME_MENU_PARAGRAPH, &HOME_MENU_TITLE),
        frame.size(),
    );
    if let Some(num) = app.selected {
        match num {
            0 => {
                app.set_command(Command::Curl(Curl::new()));
                app.goto_screen(Screen::Method);
            }
            1 => {
                app.set_command(Command::Wget(Wget::new()));
                app.goto_screen(Screen::Downloads);
            }
            2 => app.goto_screen(Screen::KeysMenu),
            3 => app.goto_screen(Screen::SavedCommands),
            _ => {}
        }
    }
}
