use tui::backend::Backend;
use tui::widgets::ListState;
use tui::Frame;

use crate::app::App;
use crate::display::displayopts::DisplayOpts;
use crate::display::inputopt::InputOpt;
use crate::screens::screen::Screen;
use crate::ui::widgets::boxes::default_rect;
use crate::ui::widgets::menu::menu_widget;

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
    frame.render_widget(menu_widget(), frame.size());
    match app.selected {
        Some(num) => match num {
            // Add a URL,
            0 => app.goto_screen(Screen::InputMenu(InputOpt::URL)),
            // Auth
            1 => app.goto_screen(Screen::Authentication),
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
            // Execute command
            7 => match app.command.as_mut().unwrap().execute() {
                Ok(()) => {
                    if app.command.as_ref().unwrap().get_response().is_some() {
                        app.response = app.command.as_ref().unwrap().get_response().clone();
                        app.goto_screen(Screen::Response(app.response.clone().unwrap()));
                    } else {
                        app.goto_screen(Screen::Error("Unable To Retreve Response...".to_string()));
                    }
                }
                Err(e) => {
                    app.goto_screen(Screen::Error(e.to_string()));
                }
            },

            _ => {}
        },
        None => {}
    }
}
