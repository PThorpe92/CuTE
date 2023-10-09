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

// All Input Type Screens
pub mod input;

// Auth Screen
pub mod auth;

pub mod tui;

pub mod render;

use ::tui::prelude::{Constraint, Direction, Layout, Rect};
pub use screen::Screen;

pub mod saved_commands;

pub fn small_alert_box(r: Rect) -> Rect {
    centered_rect(70, 60, r)
}

pub fn default_rect(r: Rect) -> Rect {
    centered_rect(70, 60, r)
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
        .direction(Direction::Vertical) // Set the direction to horizontal
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
