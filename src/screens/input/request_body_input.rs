use crate::app::{App, InputMode};
use crate::display::inputopt::InputOpt;
use crate::display::AppOptions;
use crate::request::curl::Method;
use crate::screens::{default_rect, small_alert_box, Screen};
use tui::text::{Line, Text};
use tui::widgets::{Block, Borders, Paragraph};
use tui::{
    prelude::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Span,
    Frame,
};

use super::input::render_input_with_prompt;

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
        .split(default_rect(frame.size()));
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
    let prompt = Text::from(
        "Enter your Request body,\n press ESC to exit Insert Mode\n then press Enter to submit",
    );
    match app.command.get_method() {
        Some(Method::Get | Method::Delete | Method::Head) => {
            app.goto_screen(&Screen::RequestMenu(String::from(
                "Alert: Request Bodies are not allowed for this HTTP method",
            )));
        }
        Some(_) => {}
        None => {
            app.goto_screen(&Screen::RequestMenu(String::from(
                "Alert: Please select a HTTP method first",
            )));
        }
    }

    let msg = Paragraph::new(Line::from(msg));
    let prompt = prompt.patch_style(style);
    frame.render_widget(msg, small_alert_box(frame.size()));
    render_input_with_prompt(frame, prompt);

    let width = chunks[0].width.max(3) - 3;
    let scroll = app.input.visual_scroll(width as usize);
    if app
        .command
        .get_request_body()
        .is_some_and(|s| !s.is_empty())
        && app.input.value().is_empty()
    {
        let body = app.command.get_request_body().unwrap();
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
        app.add_app_option(AppOptions::RequestBody(app.messages[0].clone()));
        app.goto_screen(&Screen::RequestMenu(String::new()));
        app.messages.clear();
    }
}
