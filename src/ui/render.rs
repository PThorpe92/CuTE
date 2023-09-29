use tui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, ListState, Paragraph},
    Frame,
};

use crate::app::App;
use crate::display::displayopts::DisplayOpts;
use crate::display::inputopt::InputOpt;
use crate::display::menuopts::METHOD_MENU_OPTIONS;
use crate::request::command::Command;
use crate::request::curl::Curl;
use crate::screens::input::render_input_screen;
use crate::screens::keys::handle_api_key_screen;
use crate::screens::screen::Screen;

pub static CURL: &str = "curl";
pub static WGET: &str = "wget";
pub static CUSTOM: &str = "custom";

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    if app.response.is_none() {
        // Render Display Options *******************************************
        let mut display_opts = String::new();
        app.opts.iter().for_each(|opt| match opt {
            DisplayOpts::Verbose => {
                display_opts.push_str("- Verbose\n");
            }
            DisplayOpts::URL(url) => {
                let url_str = format!("- URL: {}\n", &url);
                display_opts.push_str(url_str.as_str());
            }
            _ => {}
        });
        let final_opts = display_opts.clone();
        let opts = Paragraph::new(final_opts.as_str())
            .block(
                Block::default()
                    .title("Options")
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Yellow))
            .alignment(Alignment::Left);
        let area = small_rect(frame.size());
        frame.render_widget(opts, area);
        // ******************************************************************
    } else {
        let area = small_rect(frame.size());
        let response = app.response.clone().unwrap();
        let paragraph = Paragraph::new(Text::from(response.as_str()))
            .style(Style::default().fg(Color::Yellow).bg(Color::Black))
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, area);
    }

    // Render the current screen
    match &app.current_screen.clone() {
        // HOME SCREEN ******************************************************
        Screen::Home => {}

        // METHOD SCREEN ****************************************************
        Screen::Method => {
            let area = default_rect(frame.size());
            let new_list = app.current_screen.get_list();
            let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
            app.items = app.current_screen.get_opts();
            app.state = Some(state.clone());
            app.state.as_mut().unwrap().select(Some(app.cursor));
            frame.set_cursor(0, app.cursor as u16);
            frame.render_stateful_widget(new_list, area, &mut state);
            frame.render_widget(menu_paragraph(), frame.size());
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

        Screen::Downloads => {}
        // KEYS SCREEN **********************************************
        Screen::Keys => {
            handle_api_key_screen(app, frame);
        }

        // REQUEST MENU SCREEN **********************************************
        Screen::RequestMenu(_) => {
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

        // SUCESSS SCREEN *********************************************************
        Screen::Success => {
            let area = default_rect(frame.size());
            app.items = app.current_screen.get_opts();
            frame.set_cursor(0, app.cursor as u16);
            frame.render_widget(menu_paragraph(), area);
        }

        // INPUT MENU SCREEN *****************************************************
        Screen::InputMenu(opt) => {
            render_input_screen(app, frame, opt.clone());
        }

        // RESPONSE SCREEN ******************************************************
        Screen::Response(resp) => {
            let area = default_rect(small_alert_box(frame.size()));
            let new_list = app.current_screen.get_list();
            let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
            let paragraph = Paragraph::new(Text::from(resp.as_str()))
                .style(Style::default().fg(Color::Yellow).bg(Color::Black))
                .alignment(Alignment::Center);
            if !app.items.is_empty() {
                app.items.clear();
            }
            app.items = app.current_screen.get_opts();
            app.state = Some(state.clone());
            app.state.as_mut().unwrap().select(Some(app.cursor));
            frame.set_cursor(0, app.cursor as u16);
            frame.render_stateful_widget(new_list, area, &mut state);
            let area_2 = small_alert_box(frame.size());
            frame.render_widget(paragraph, area_2);
            match app.selected {
                Some(num) => match num {
                    0 => {
                        app.goto_screen(Screen::InputMenu(InputOpt::Output));
                    }
                    1 => {
                        app.goto_screen(Screen::Commands);
                    }
                    2 => {
                        app.goto_screen(Screen::ViewBody);
                    }
                    _ => {}
                },
                None => {}
            }
        }

        // VIEW BODY ********************************************************************
        Screen::ViewBody => {
            // screen with only the body of the response
            app.items.clear();
            let area = small_rect(frame.size());
            let response = app.response.clone().unwrap();
            let paragraph = Paragraph::new(Text::from(response.as_str()))
                .style(Style::default().fg(Color::Yellow).bg(Color::Black))
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, area);
        }
        _ => {}
    }
}

/* Never Used
fn success_paragraph() -> Paragraph<'static> {
    Paragraph::new("Command successfully saved\n")
        .block(
            Block::default()
                .title("cURL-TUI")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .alignment(Alignment::Center)
}
 */
