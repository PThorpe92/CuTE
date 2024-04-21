use ::tui::prelude::{Color, Text};
use ::tui::prelude::{Constraint, Direction, Frame, Layout, Rect};
use ::tui::style::Style;
use ::tui::widgets::{Block, Borders, Paragraph, Wrap};
pub mod screen;
// Method Select Screens
pub mod cookies;
pub mod method;
// Request Select Screens
pub mod request;
// Response Screen
pub mod more_flags;
pub mod response;
// All Input Type Screens
pub mod input;
// Auth Screen
pub mod auth;
pub mod render;
pub mod saved_keys;
pub use screen::Screen;
pub mod collections;
pub mod error;
pub mod headers;
pub mod saved_commands;
pub fn error_alert_box(frame: &mut Frame<'_>, error_message: &str) -> Rect {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),      // This will be the alert box
            Constraint::Percentage(70), // This aligns the main screen perfectly with the bottom
            Constraint::Percentage(27),
        ])
        .split(frame.size());

    // Render the alert box
    let alert_box = layout[0];
    let alert_text_chunk = match error_message.starts_with("Error:") {
        true => Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Red).fg(Color::White))
            .title("Alert"),
        false => Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::LightBlue).fg(Color::White))
            .title("Success"),
    };
    frame.render_widget(alert_text_chunk, alert_box);
    let text = Text::styled(
        error_message,
        Style::default().fg(Color::White).bg(Color::Red),
    );
    let alert_text_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(alert_box)[0];
    frame.render_widget(
        Paragraph::new(text)
            .alignment(::tui::prelude::Alignment::Center)
            .wrap(Wrap { trim: true }),
        alert_text_chunk,
    );

    let main_box = layout[1];
    centered_rect(main_box, ScreenArea::Top)
}

#[derive(Debug, Copy, Clone)]
pub enum ScreenArea {
    Top = 0,
    Center = 1,
    Bottom = 2,
}
// we want to center the rectangle in the middle of the screen
// but we want the padding on the bottom to also be it's own area
// so we split the screen into 3 parts, the top and bottom are padding
pub fn centered_rect(r: Rect, area: ScreenArea) -> Rect {
    let chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .vertical_margin(2)
        .split(r)[1];
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(chunk)[area as usize]
}

// **********************************************************************************
pub fn single_line_input_box(frame_size: Rect) -> Rect {
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
        .split(frame_size);
    chunks[0]
}
