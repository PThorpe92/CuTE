use std::rc::Rc;

use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::prelude::Backend;
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;

use crate::app::App;

pub fn handle_url_input_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let layout_chunks = create_layout(frame);

    let title_paragraph = create_screen_title("URL Input");
    let input_block = create_input_block(app);

    frame.render_widget(title_paragraph, layout_chunks[0]);
    frame.render_widget(input_block, layout_chunks[1]);
}

fn create_layout<B: Backend>(frame: &mut Frame<'_, B>) -> Rc<[Rect]> {
    let parent_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3), // Title Area
                Constraint::Length(3), // Input Area
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(frame.size());

    parent_chunks
}

fn create_screen_title<'a>(title_str: &'a str) -> Paragraph<'a> {
    let title_block = Block::default().style(Style::default());
    let title = Paragraph::new(Text::styled(title_str, Style::default().fg(Color::Green)))
        .block(title_block);
    title
}

fn create_input_block<'a>(app: &'a mut App) -> Paragraph<'a> {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        app.input.value(),
        Style::default().fg(Color::Green),
    ))
    .block(title_block);
    title
}
