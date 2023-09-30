/// Renders a screen we can grab input from, pass in the appropriate designation for the input
pub fn render_input_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, opt: InputOpt) {
    match opt {
        InputOpt::URL => render_default_input(app, frame, opt),
        InputOpt::Headers => render_headers_input(app, frame, opt),
        InputOpt::RecursiveDownload => render_recursive_download_input(app, frame, opt),
        _ => render_default_input(app, frame, opt),
    }
}

pub fn render_headers_input<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, opt: InputOpt) {
    let header_prompt = Text::from(Line::from(
        "MUST be \"Key:Value\" pair and press Enter \n Example: Content-Type: application/json",
    ));
    render_input_with_prompt(app, frame, header_prompt, opt);
}

pub fn render_input_with_prompt<B: Backend>(
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

pub fn render_recursive_download_input<B: Backend>(
    app: &mut App,
    frame: &mut Frame<'_, B>,
    opt: InputOpt,
) {
    let header_prompt = Text::from(Line::from(
        "Enter the recursion level and press Enter \n Example: 2",
    ));
    render_input_with_prompt(app, frame, header_prompt, opt);
}

pub fn render_default_input<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, opt: InputOpt) {
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
                    .set_headers(headers.iter().map(|x| x.to_string()).collect());
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
                    .set_rec_download_level(recursion_level);
            }
            _ => {}
        }
    }
}
