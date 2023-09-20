use crate::app::{
    App, Command, DisplayOpts, InputMode, InputOpt, Screen, METHOD_MENU_OPTIONS,
};
use crate::curl::Curl;
use crate::wget::Wget;
use crate::{Request, GET};

use tokio::runtime::{Runtime};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, ListState, Paragraph},
    Frame,
};
pub static CURL: &str = "curl";
pub static WGET: &str = "wget";
pub static CUSTOM: &str = "custom";

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let mut display_opts = String::new();
    app.opts.iter().for_each(|opt| match opt {
        DisplayOpts::Verbose => {
            display_opts.push_str("Verbose\n");
        }
        DisplayOpts::URL(url) => {
            let url_str = format!("URL: {}\n", url.clone());
            display_opts.push_str(url_str.as_str());
        }
        DisplayOpts::Response(resp) => {
            let response = format!("Response: {}\n", resp.clone());
            display_opts.push_str(response.as_str());
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
    if !app.opts.is_empty() {
        frame.render_widget(opts, area);
    }
    match &app.current_screen.clone() {
        Screen::Home => {
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
                    app.goto_screen(Screen::Method(String::from(CURL)));
                }
                Some(1) => {
                    app.goto_screen(Screen::Method(String::from(WGET)));
                }
                Some(2) => {
                    app.goto_screen(Screen::Method(String::from(CUSTOM)));
                }
                Some(3) => {
                    app.goto_screen(Screen::Keys);
                }
                Some(_) => {}
                None => {}
            }
        }

        Screen::Method(cmd) => {
            let area = default_rect(frame.size());
            let new_list = app.current_screen.get_list();
            let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
            app.items = app.current_screen.get_opts();
            app.state = Some(state.clone());
            app.state.as_mut().unwrap().select(Some(app.cursor));
            frame.set_cursor(0, app.cursor as u16);
            frame.render_stateful_widget(new_list, area, &mut state);
            frame.render_widget(menu_paragraph(), frame.size());
            match cmd.as_str() {
                "curl" => app.command = Some(Command::Curl(Curl::new())),
                "wget" => app.command = Some(Command::Wget(Wget::new())),
                "custom" => app.command = Some(Command::Custom(Request::default())),
                _ => app.command = Some(Command::Custom(Request::default())),
            }
            match app.selected {
                Some(num) => {
                    app.command.as_mut().unwrap().set_method(String::from(GET));
                    app.goto_screen(Screen::RequestMenu(String::from(
                        METHOD_MENU_OPTIONS[num].clone(),
                    )));
                }
                None => {}
            }
        }
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
                        app.add_display_option(DisplayOpts::Verbose);
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
                    // Execute
                    6 => {
                        let response = app.command.as_mut().unwrap().execute();
                        let rt = Runtime::new().expect("Failed to create runtime");
                        let result = rt.block_on(response); // TODO: this is blocking, whole thing
                                                            // needs a refactor so we aren't doing what __should__ be an async call in a
                                                            // looping render function like a noob
                        app.goto_screen(Screen::Response(
                            result.unwrap_or(String::from("Internal Error with response")),
                        ));
                    }
                    // Save the command
                    7 => {
                        app.goto_screen(Screen::Saved);
                        app.selected = None;
                    }
                    // Specify recursive download (wget)
                    9 => {
                        app.selected = None;
                        app.goto_screen(Screen::InputMenu(InputOpt::RecursiveDownload));
                    }
                    _ => {}
                },
                None => {}
            }
        }
        Screen::Success => {
            let area = default_rect(frame.size());
            app.items = app.current_screen.get_opts();
            frame.set_cursor(0, app.cursor as u16);
            frame.render_widget(menu_paragraph(), area);
        }
        Screen::InputMenu(opt) => {
            render_input_screen(app, frame, opt.clone());
        }
        Screen::Response(resp) => {
            app.command.as_mut().unwrap().set_response(resp.clone());
            let area = default_rect(frame.size());
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
            let area = small_rect(frame.size());
            frame.render_widget(paragraph, area);
            match app.selected {
                Some(num) => match num {
                    0 => {
                        app.goto_screen(Screen::InputMenu(InputOpt::Output));
                    }
                    1 => {
                        app.goto_screen(Screen::Saved);
                    }
                    _ => {}
                },
                None => {}
            }
        }
        _ => {}
    }
}

/// Renders a screen we can grab input from, pass in the appropriate desination for the input
fn render_input_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, opt: InputOpt) {
    match opt {
        InputOpt::Headers => render_headers_input(app, frame, opt),
        InputOpt::RecursiveDownload => render_recursive_download_input(app, frame, opt),
        _ => render_default_input(app, frame, opt),
    }
}

fn render_headers_input<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, opt: InputOpt) {
    let header_prompt = Text::from(Line::from(
        "MUST be \"Key:Value\" pair and press Enter \n Example: Content-Type: application/json",
    ));
    render_input_with_prompt(app, frame, header_prompt, opt);
}

fn render_recursive_download_input<B: Backend>(
    app: &mut App,
    frame: &mut Frame<'_, B>,
    opt: InputOpt,
) {
    let header_prompt = Text::from(Line::from(
        "Enter the recursion level and press Enter \n Example: 2",
    ));
    render_input_with_prompt(app, frame, header_prompt, opt);
}

fn render_default_input<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, opt: InputOpt) {
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

    let mut header_prompt = Text::from("Enter a value and press Enter");
    header_prompt.patch_style(style);

    render_input_with_prompt(app, frame, header_prompt, opt.clone());

    let width = chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor

    let scroll = app.input.visual_scroll(width as usize);
    let input = Paragraph::new(app.input.value())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .scroll((0, scroll as u16))
        .block(Block::default().borders(Borders::ALL).title("Input"));
    frame.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => frame.set_cursor(
            chunks[1].x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            chunks[1].y + 1,
        ),
    }

    if !app.messages.is_empty() {
        match opt {
            InputOpt::URL => {
                app.command
                    .as_mut()
                    .unwrap()
                    .set_url(app.messages[0].clone());
                app.input_mode = InputMode::Normal;
                app.add_display_option(DisplayOpts::URL(app.messages[0].clone()));
                app.current_screen = Screen::RequestMenu(String::new());
                app.messages.remove(0);
            }
            InputOpt::Headers => {
                let headers = app
                    .messages
                    .get(0)
                    .unwrap()
                    .split(':')
                    .collect::<Vec<&str>>();
                let cpy = (
                    String::from(headers[0].clone()),
                    String::from(headers[1].clone()),
                );
                app.command
                    .as_mut()
                    .unwrap()
                    .add_headers((headers[0].to_string(), headers[1].to_string()));
                app.add_display_option(DisplayOpts::Headers(cpy));
                app.current_screen = Screen::RequestMenu(String::new());
                app.messages.remove(0);
            }
            InputOpt::Output => {
                app.command
                    .as_mut()
                    .unwrap()
                    .set_outfile(app.messages[0].clone());
                app.add_display_option(DisplayOpts::Outfile(app.messages[0].clone()));
                app.messages.remove(0);
                app.goto_screen(Screen::RequestMenu(String::new()));
            }
            InputOpt::Execute => {
                // This means they have executed the command, and want to write to a file
                app.command
                    .as_mut()
                    .unwrap()
                    .set_outfile(app.messages[0].clone());
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
                let recursion_level = app.messages[0].parse::<usize>().unwrap();
                app.command
                    .as_mut()
                    .unwrap()
                    .set_rec_download(recursion_level);
            }
            _ => {}
        }
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
