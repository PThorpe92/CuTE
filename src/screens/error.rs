use crate::app::App;
use crate::screens::{default_rect, small_alert_box};
use tui::layout::{Constraint, Layout};
use tui::prelude::Direction;
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::{Block, Borders, ListState, Paragraph, Wrap};
use tui::Frame;

fn err_box(frame: &mut Frame<'_>, error_msg: String) {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - 40) / 2),
                Constraint::Percentage(40),
                Constraint::Percentage((100 - 40) / 2),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let boundbox = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1];

    let err_box_chunk = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Red).fg(Color::White))
        .title("CuTE Error:");
    frame.render_widget(err_box_chunk, boundbox);

    let innerbox = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(50)].as_ref())
        .split(boundbox)[1];

    frame.render_widget(
        Paragraph::new(Text::from(error_msg))
            .alignment(::tui::prelude::Alignment::Center)
            .wrap(Wrap { trim: true }),
        innerbox,
    );
}

pub fn handle_error_screen(app: &mut App, frame: &mut Frame<'_>, error_msg: String) {
    let area = default_rect(small_alert_box(frame.size()));
    let new_list = app.current_screen.get_list(None);
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    if !app.items.is_empty() {
        app.items.clear();
    }
    app.items = app.current_screen.get_opts(None);
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(new_list, area, &mut state);

    err_box(frame, error_msg);
}
