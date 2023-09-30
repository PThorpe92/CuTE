/*
* This file contains the functions that are used to create the different boxes
* and rectangles and such that are used in the UI
 */

use tui::layout::{Constraint, Direction, Layout, Rect};

pub fn small_alert_box(r: Rect) -> Rect {
    centered_rect(70, 60, r)
}

pub fn default_rect(r: Rect) -> Rect {
    centered_rect(70, 60, r)
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

pub fn small_rect(r: Rect) -> Rect {
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
