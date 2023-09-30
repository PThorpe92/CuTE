use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, ListState, Paragraph},
    Frame,
};

use crate::app::{App, InputMode};
use crate::display::displayopts::DisplayOpts;
use crate::display::inputopt::InputOpt;
use crate::display::menuopts::METHOD_MENU_OPTIONS;
use crate::request::command::Command;
use crate::request::curl::{AuthKind, Curl};
use crate::request::wget::Wget;
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
            DisplayOpts::RecDownload(num) => {
                let rec_str = format!("- Recursive Download depth: {}\n", num);
                display_opts.push_str(rec_str.as_str());
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
        Screen::Home => {
            let new_list = app.current_screen.get_list();
            let area = centered_rect(70, 60, frame.size());
            let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
            app.state = Some(state.clone());
            app.state.as_mut().unwrap().select(Some(app.cursor));
            frame.set_cursor(0, app.cursor as u16);
            frame.render_stateful_widget(new_list, area, &mut state);
            frame.render_widget(menu_paragraph(), frame.size());
            if let Some(num) = app.selected {
                match num {
                    0 => {
                        app.command = Some(Command::Curl(Curl::new()));
                        app.goto_screen(Screen::Method);
                    }
                    1 => {
                        app.command = Some(Command::Wget(Wget::new()));
                        app.goto_screen(Screen::Downloads);
                    }
                    2 => app.goto_screen(Screen::Keys),
                    3 => app.goto_screen(Screen::Commands),
                    _ => {}
                }
            }
        }

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

        Screen::Downloads => {
            let area = default_rect(frame.size());
            let list = app.current_screen.get_list();
            let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
            app.items = app.current_screen.get_opts();
            app.state = Some(state.clone());
            app.state.as_mut().unwrap().select(Some(app.cursor));
            frame.set_cursor(0, app.cursor as u16);
            frame.render_stateful_widget(list, area, &mut state);
            frame.render_widget(menu_paragraph(), frame.size());
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
                    3 => match app.command.as_mut().unwrap().execute() {
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
        // KEYS SCREEN **********************************************
        Screen::Keys => {
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
            frame.render_widget(api_key_paragraph(), frame.size());
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
                },
                None => {}
            }
        }
        // AUTHENTICATION SCREEN *************************************************
        Screen::Authentication => {
            let area = default_rect(frame.size());
            let new_list = app.current_screen.get_list();
            let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
            app.items = app.current_screen.get_opts();
            app.state = Some(state.clone());
            app.state.as_mut().unwrap().select(Some(app.cursor));
            frame.set_cursor(0, app.cursor as u16);
            frame.render_stateful_widget(new_list, area, &mut state);
            frame.render_widget(menu_paragraph(), frame.size());
            if let Some(num) = app.selected {
                match num {
                    0 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthKind::Basic(
                        String::new(),
                    )))),
                    1 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthKind::Bearer(
                        String::new(),
                    )))),
                    2 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthKind::Digest(
                        String::new(),
                    )))),
                    3 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthKind::AwsSigv4(
                        String::new(),
                    )))),
                    4 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthKind::Spnego(
                        String::new(),
                    )))),
                    5 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthKind::Kerberos(
                        String::new(),
                    )))),
                    6 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthKind::Ntlm(
                        String::new(),
                    )))),
                    _ => {}
                }
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

/// Renders a screen we can grab input from, pass in the appropriate designation for the input
fn render_input_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, opt: InputOpt) {
    match opt {
        InputOpt::URL => {
            let text = Text::from(Line::from("Enter a URL and press Enter"));
            render_default_input(app, frame, text, opt);
        }
        InputOpt::Headers => {
            let text = Text::from(Line::from(
                "MUST be \"Key:Value\" pair and press Enter \n Example: Content-Type: application/json",
            ));
            render_default_input(app, frame, text, opt);
        }
        InputOpt::RecursiveDownload => {
            let text = Text::from("Enter the recursion level and press Enter \n Example: 2");
            render_default_input(app, frame, text, opt);
        }
        _ => {
            let text = Text::from("Enter a value and press Enter");
            render_default_input(app, frame, text, opt);
        }
    }
}

fn render_default_input<B: Backend>(
    app: &mut App,
    frame: &mut Frame<'_, B>,
    mut prompt: Text,
    opt: InputOpt,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(frame.size());
    let (_msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to submit."),
            ],
            Style::default(),
        ),
    };

    prompt.patch_style(style);

    render_input_with_prompt(app, frame, prompt, opt.clone());

    let width = chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor

    let scroll = app.input.visual_scroll(width as usize);
    let input = Paragraph::new(app.input.value())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .scroll((0, scroll as u16))
        .block(Block::default().borders(Borders::ALL).title("Enter Input"));
    frame.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => frame.set_cursor(
            chunks[1].x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            chunks[1].y + 1,
        ),
    }

    if !app.messages.is_empty() {
        app.input_mode = InputMode::Normal;
        parse_input(app.messages[0].clone(), opt, app);
        app.messages.remove(0);
    }
}

fn parse_input(message: String, opt: InputOpt, app: &mut App) {
    match opt {
        InputOpt::URL => {
            app.command.as_mut().unwrap().set_url(message.clone());
            app.add_display_option(DisplayOpts::URL(message.clone()));
            match app.command.as_ref().unwrap() {
                Command::Curl(_) => app.current_screen = Screen::RequestMenu(String::new()),
                Command::Wget(_) => app.current_screen = Screen::Downloads,
            }
        }
        InputOpt::RequestBody => {
            // This means they should be pasting in a large request body...
            //  not sure how to handle this yet.
        }
        InputOpt::Verbose => {
            // This shouldn't have sent them to this screen...
        }

        InputOpt::Headers => {
            let headers = message.split(':').collect::<Vec<&str>>();
            let cpy = (
                String::from(headers[0].clone()),
                String::from(headers[1].clone()),
            );
            app.command
                .as_mut()
                .unwrap()
                .set_headers(headers.iter().map(|x| x.to_string()).collect());
            app.add_display_option(DisplayOpts::Headers(cpy));
            app.current_screen = Screen::RequestMenu(String::new());
        }

        InputOpt::Output => {
            app.input_mode = InputMode::Normal;
            app.command.as_mut().unwrap().set_outfile(&message);
            app.add_display_option(DisplayOpts::Outfile(message.clone()));
            app.goto_screen(Screen::RequestMenu(String::new()));
        }

        InputOpt::Execute => {
            // This means they have executed the command, and want to write to a file
            app.command.as_mut().unwrap().set_outfile(&message);
            match app.command.as_mut().unwrap().write_output() {
                Ok(_) => {
                    app.goto_screen(Screen::Success);
                }
                Err(e) => {
                    app.goto_screen(Screen::Error(e.to_string()));
                }
            }
            app.goto_screen(Screen::Home);
        }

        InputOpt::RecursiveDownload => {
            let recursion_level = message.parse::<usize>().unwrap();
            app.command
                .as_mut()
                .unwrap()
                .set_rec_download_level(recursion_level);
            app.add_display_option(DisplayOpts::RecDownload(recursion_level));
            app.current_screen = Screen::Downloads;
        }

        InputOpt::Auth(auth) => match auth {
            AuthKind::Basic(_) => {
                app.command
                    .as_mut()
                    .unwrap()
                    .set_auth(AuthKind::Basic(message.clone()));
                app.add_display_option(DisplayOpts::Auth(message));
            }
            AuthKind::Bearer(_) => {
                app.command
                    .as_mut()
                    .unwrap()
                    .set_auth(AuthKind::Bearer(message.clone()));
                app.add_display_option(DisplayOpts::Auth(message));
            }
            AuthKind::Digest(_) => {
                app.command
                    .as_mut()
                    .unwrap()
                    .set_auth(AuthKind::Digest(message.clone()));
                app.add_display_option(DisplayOpts::Auth(message));
            }
            AuthKind::AwsSigv4(_) => {
                app.command
                    .as_mut()
                    .unwrap()
                    .set_auth(AuthKind::AwsSigv4(message.clone()));
                app.add_display_option(DisplayOpts::Auth(message));
            }
            AuthKind::Spnego(_) => {
                app.command
                    .as_mut()
                    .unwrap()
                    .set_auth(AuthKind::Spnego(message.clone()));
                app.add_display_option(DisplayOpts::Auth(message));
            }
            AuthKind::Kerberos(_) => {
                app.command
                    .as_mut()
                    .unwrap()
                    .set_auth(AuthKind::Kerberos(message.clone()));
                app.add_display_option(DisplayOpts::Auth(message));
            }
            AuthKind::Ntlm(_) => {
                app.command
                    .as_mut()
                    .unwrap()
                    .set_auth(AuthKind::Ntlm(message.clone()));
                app.add_display_option(DisplayOpts::Auth(message));
            }
            AuthKind::NtlmWb(_) => {
                app.command
                    .as_mut()
                    .unwrap()
                    .set_auth(AuthKind::NtlmWb(message.clone()));
                app.add_display_option(DisplayOpts::Auth(message));
            }
            AuthKind::None => {}
        },
    }
}

fn render_input_with_prompt<B: Backend>(
    _app: &mut App,
    frame: &mut Frame<'_, B>,
    prompt: Text,
    _opt: InputOpt,
) {
    // Render the input with the provided prompt
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let message = Paragraph::new(prompt);
    frame.render_widget(message, chunks[0]);
}

fn menu_paragraph() -> Paragraph<'static> {
    Paragraph::new("\nPress q to exit \n Press Enter to select \n Please select a Menu item\n")
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

fn api_key_paragraph() -> Paragraph<'static> {
    Paragraph::new(
        "Create / Edit / Delete API Keys and tokens.\n
                    Press q to exit \n Press Enter to select \n Please select a Menu item\n",
    )
    .block(
        Block::default()
            .title("API Key Manager")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Cyan).bg(Color::Black))
    .alignment(Alignment::Center)
}

fn small_rect(r: Rect) -> Rect {
    let layout = Layout::default()
        .direction(Direction::Vertical) // Set the direction to horizontal
        .constraints(vec![
            Constraint::Percentage(85), // Occupy 85% of the available space
            Constraint::Percentage(15), // Occupy 15% of the available space
        ])
        .split(r);
    // Now, `layout` contains the two Rects based on the constraints
    layout[1]
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = tui::layout::Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    tui::layout::Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn small_alert_box(r: Rect) -> Rect {
    centered_rect(70, 60, r)
}

fn default_rect(r: Rect) -> Rect {
    centered_rect(70, 60, r)
}
