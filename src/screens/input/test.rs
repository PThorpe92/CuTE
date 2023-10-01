use std::rc::Rc;

use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;

use crate::app::App;

fn create_layout<B: Backend>(frame: &mut Frame<'_, B>) -> (Rc<[Rect]>, Rc<[Rect]>, Rc<[Rect]>) {
    let parent_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Title Area
                Constraint::Length(3), // Input Area
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let body_column_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(parent_chunks[2]);

    let left_col_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(body_column_chunks[0]);

    let right_col_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(body_column_chunks[1]);

    (parent_chunks, right_col_chunks, left_col_chunks)
}

fn create_screen_title<'a>(title_str: &'a str) -> Paragraph<'a> {
    let title_block = Block::default().style(Style::default());

    let title = Paragraph::new(Text::styled(title_str, Style::default().fg(Color::Green)))
        .block(title_block);
    title
}

fn create_input_block<'a>(app: &'a mut App) -> Paragraph<'a> {
    let input_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let input = Paragraph::new(Text::styled(
        app.input.value(),
        Style::default().fg(Color::Green),
    ))
    .block(input_block);
    input
}

pub fn TextInput<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // Create Render Objects & Layout
    let (parent_chunks, right_col_chunks, _left_col_chunks) = create_layout(frame);
    let title = create_screen_title("Test Title");
    let input = create_input_block(app);
    // Render Widgets
    frame.render_widget(title, parent_chunks[0]); // Title Widget
    frame.render_widget(input, right_col_chunks[1]); // Input Widget
}
