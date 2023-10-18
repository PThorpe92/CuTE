pub mod screen;

// Home Screen
pub mod home;

// Downloads Screen
pub mod downloads;

// Method Select Screens
pub mod method;

// Request Select Screens
pub mod request;

// Response Screen
pub mod response;

pub mod more_flags;

// All Input Type Screens
pub mod input;

// Auth Screen
pub mod auth;

pub mod tui;

pub mod render;

pub mod saved_keys;

use ::tui::prelude::{Backend, Constraint, Direction, Frame, Layout, Rect};
use ::tui::prelude::{Color, Text};
use ::tui::style::Style;
use ::tui::widgets::{Block, Borders, Paragraph, Wrap};
pub use screen::Screen;

pub mod saved_commands;

pub fn small_alert_box(r: Rect) -> Rect {
    centered_rect(70, 60, r)
}

pub fn default_rect(r: Rect) -> Rect {
    centered_rect(70, 60, r)
}

pub fn error_alert_box<B: Backend>(frame: &mut Frame<'_, B>, error_message: &str) -> Rect {
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
    let alert_text_chunk = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Red).fg(Color::White))
        .title("Alert");
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

    // Render the centered box
    let main_box = layout[1];
    centered_rect(70, 60, main_box)
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
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

pub fn small_rect(r: Rect) -> Rect {
    let layout = Layout::default()
        .direction(Direction::Vertical) // Set the direction
        .constraints(vec![
            Constraint::Percentage(78), // This aligns the main screen perfectly with the bottom
            Constraint::Percentage(22),
        ])
        .split(r);
    // Now, `layout` contains the two Rects based on the constraints
    layout[1]
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
