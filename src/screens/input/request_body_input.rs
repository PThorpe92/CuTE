use crate::app::{App, InputMode};
use crate::display::inputopt::InputOpt;
use crate::display::AppOptions;
use crate::screens::{centered_rect, Screen, ScreenArea};
use tui::style::Styled;
use tui::text::Line;
use tui::widgets::{Block, Borders, Paragraph};
use tui::{
    prelude::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Span,
    Frame,
};

pub fn handle_req_body_input_screen(app: &mut App, frame: &mut Frame<'_>, _opt: InputOpt) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(centered_rect(frame.size(), ScreenArea::Center));
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::styled("Press 'h'", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("to go back..."),
                Span::styled("Press 'i'", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("to start editing."),
            ],
            Style::default(),
        ),
        InputMode::Editing => (
            vec![
                Span::styled("Press Esc\n", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing...\n"),
                Span::styled("Press Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to submit."),
            ],
            Style::default(),
        ),
    };
    let prompt = vec![
        Line::raw("Enter your Request body Or the path to a file containing the body."),
        Line::raw(" "),
        Line::raw("Example: {\"key\", \"value\"} (no outside quotes needed)\n"),
        Line::raw(" "),
        Line::raw("a .json filepath will automatically add Content-Type Header"),
        Line::raw(" "),
        Line::raw("then press Enter to submit"),
    ];
    if !app.command.method.needs_reset() {
        app.goto_screen(&Screen::RequestMenu(Some(InputOpt::RequestError(
            String::from("Error: Request Bodies are not allowed for this HTTP method"),
        ))));
    }

    let msg = Paragraph::new(Line::from(msg));
    let prompt = Paragraph::new(prompt).set_style(style);
    frame.render_widget(msg, centered_rect(frame.size(), ScreenArea::Top));
    frame.render_widget(&prompt, chunks[0]);

    let width = chunks[0].width.max(3) - 3;
    let scroll = app.input.visual_scroll(width as usize);
    if app.get_request_body().is_some_and(|s| !s.is_empty()) && app.input.value().is_empty() {
        let body = app.get_request_body().unwrap();
        for ch in body.chars() {
            app.input.handle(tui_input::InputRequest::InsertChar(ch));
        }
        app.command.set_request_body("");
    }
    let input = Paragraph::new(app.input.value())
        .wrap(tui::widgets::Wrap { trim: (true) })
        .style(match app.input_mode {
            InputMode::Normal => app.config.get_style(),
            InputMode::Editing => Style::default().fg(app.config.get_outline_color()),
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

    // we have input (the user has typed something and pressed Enter while in insert mode)
    if !app.messages.is_empty() {
        let input = app.messages[0].clone();
        if app.messages[0].ends_with(".json") && std::path::Path::new(&input).exists() {
            let body = std::fs::read_to_string(input).unwrap();
            app.command.set_request_body(&body);
            app.add_app_option(AppOptions::RequestBody(body));
            app.add_app_option(AppOptions::ContentHeaders(
                crate::display::HeaderKind::ContentType("application/json".to_string()),
            ));
            app.goto_screen(&Screen::RequestMenu(None));
            app.messages.clear();
            return;
        }
        app.add_app_option(AppOptions::RequestBody(app.messages[0].clone()));
        app.goto_screen(&Screen::RequestMenu(None));
        app.messages.clear();
    }
}
