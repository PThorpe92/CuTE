use tui::backend::Backend;
use tui::widgets::ListState;
use tui::Frame;

use crate::app::App;
use crate::display::displayopts::DisplayOpts;
use crate::display::inputopt::InputOpt;
use crate::screens::screen::Screen;
use crate::ui::widgets::{default_rect, menu_paragraph};

pub fn handle_request_menu_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
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
    frame.render_widget(menu_paragraph(), frame.size());
    match app.selected {
        Some(num) => match num {
            // Add a URL,
            0 => app.goto_screen(Screen::InputMenu(InputOpt::URL)),
            // Auth
            1 => app.goto_screen(Screen::InputMenu(InputOpt::Authentication)),
            // Headers
            2 => app.goto_screen(Screen::InputMenu(InputOpt::Headers)),
            // Verbose
            3 => {
                if app.opts.contains(&DisplayOpts::Verbose) {
                    app.opts.retain(|x| x != &DisplayOpts::Verbose);
                    app.command.as_mut().unwrap().set_verbose(false);
                } else {
                    app.add_display_option(DisplayOpts::Verbose);
                    app.command.as_mut().unwrap().set_verbose(true);
                }
                app.selected = None;
            }
            // Output file,
            4 => {
                app.goto_screen(Screen::InputMenu(InputOpt::Output));
                app.selected = None;
            }
            // Request Body
            5 => {
                app.goto_screen(Screen::InputMenu(InputOpt::RequestBody));
                app.selected = None;
            }
            // Save this command
            6 => {
                app.goto_screen(Screen::Commands);
                app.selected = None;
            }
            // Recursive download
            7 => {
                app.selected = None;
                app.goto_screen(Screen::InputMenu(InputOpt::RecursiveDownload));
            }
            // Execute command
            8 => {
                if let Ok(response) = app.command.as_mut().unwrap().execute() {
                    app.set_response(response.clone());
                    app.goto_screen(Screen::Response(response));
                }
            }
            _ => {}
        },
        None => {}
    }
}
