use tui::backend::Backend;
use tui::widgets::ListState;
use tui::Frame;

use crate::app::App;
use crate::display::inputopt::InputOpt;
use crate::request::command::Command;
use crate::request::wget::Wget;
use crate::screens::screen::Screen;
use crate::ui::widgets::{default_rect, menu_paragraph};

pub fn handle_downloads_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    app.command = Some(Command::Wget(Wget::new()));
    let area = default_rect(frame.size());
    let list = app.current_screen.get_list();
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.items = app.current_screen.get_opts();
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(list, area, &mut state);
    frame.render_widget(menu_paragraph(), frame.size());
    match app.selected {
        // Setting Recursion level
        Some(0) => app.goto_screen(Screen::InputMenu(InputOpt::RecursiveDownload)),
        // Add URL of download
        Some(1) => app.goto_screen(Screen::InputMenu(InputOpt::URL)),
        // Add file name for output/download
        Some(2) => app.goto_screen(Screen::InputMenu(InputOpt::Output)),
        // Execute command
        Some(3) => {
            if let Ok(response) = app.command.as_mut().unwrap().execute() {
                app.response = Some(response.clone());
                app.goto_screen(Screen::Success);
            }
        }
        Some(_) => {}
        None => {}
    };
}
