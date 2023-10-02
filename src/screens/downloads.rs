use tui::backend::Backend;
use tui::widgets::ListState;
use tui::Frame;

use crate::app::App;
use crate::display::inputopt::InputOpt;
use crate::screens::screen::Screen;
use crate::ui::render::{render_header_paragraph, DOWNLOAD_MENU_TITLE, HOME_MENU_PARAGRAPH};
use crate::ui::widgets::boxes::default_rect;

pub fn handle_downloads_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let area = default_rect(frame.size());
    let list = app.current_screen.get_list();
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.items = app.current_screen.get_opts();
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(list, area, &mut state);
    frame.render_widget(
        render_header_paragraph(HOME_MENU_PARAGRAPH, DOWNLOAD_MENU_TITLE),
        frame.size(),
    );

    if let Some(num) = app.selected {
        match num {
            // Setting Recursion level
            0 => {
                app.goto_screen(Screen::InputMenu(InputOpt::RecursiveDownload));
                app.selected = None;
            }
            // Add URL of download
            1 => {
                app.goto_screen(Screen::InputMenu(InputOpt::URL));
                app.selected = None;
            }
            // Add file name for output/download
            2 => {
                app.goto_screen(Screen::InputMenu(InputOpt::Output));
                app.selected = None;
                // Execute command
            }
            3 => match app.execute_command() {
                Ok(_) => {
                    if let Some(response) = app.command.as_ref().unwrap().get_response() {
                        app.response = Some(response.clone());
                        app.goto_screen(Screen::Response(response));
                    }
                }
                Err(e) => {
                    app.goto_screen(Screen::Error(e.to_string()));
                }
            },
            _ => {}
        };
    }
}
