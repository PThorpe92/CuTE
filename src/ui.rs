use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let items = [
        ListItem::new("Build and run a new cURL command"),
        ListItem::new("Build and run a new wget command"),
        ListItem::new("Build/send new custom HTTP request"),
        ListItem::new("View my stored API keys"),
        ListItem::new("View or execute my saved commands"),
    ];
    app.items = Vec::from(items.clone());
    let new_list = List::new(items)
        .block(Block::default().title("List").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol("-~>");
    let area = Rect::new(0, 0, 40, 10);
    frame.set_cursor(0, app.cursor as u16);
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));

    frame.render_stateful_widget(new_list, area, &mut state);
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));

    frame.render_widget(
        Paragraph::new(format!(
            "This is our template.\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press up and down to increment and decrement the counter respectively.\n\
                Cursor Position: {}",
            app.cursor
        ))
        .block(
            Block::default()
                .title("cURL-TUI")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .alignment(Alignment::Center),
        frame.size(),
    )
}
