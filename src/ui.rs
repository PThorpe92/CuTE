use crate::app::Screen;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::app::{App, Command};
pub static MAIN_MENU_OPTIONS: [&str; 5] = [
    "Build and run a new cURL command\n  \n",
    "Build and run a new wget command\n  \n",
    "Build/send new custom HTTP request\n  \n",
    "View my stored API keys\n  \n",
    "View or execute my saved commands\n  \n",
];
pub static COMMAND_MENU_OPTIONS: [&str; 6] = [
    "Choose an HTTP method:\n \n",
    "GET\n \n",
    "POST\n \n",
    "PUT\n \n",
    "DELETE\n \n",
    "PATCH\n \n",
];

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    match &app.current_screen {
        Screen::Home => {
            render_home(app, frame);
        }
        Screen::Command(cmd) => {
            render_command_menu(app, frame, cmd.clone());
        }
        Screen::Keys => {
            render_keys_menu(app, frame);
        }
        _ => {}
    }
}

pub fn render_home<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let choices = [
        ListItem::new(MAIN_MENU_OPTIONS[0]),
        ListItem::new(MAIN_MENU_OPTIONS[1]),
        ListItem::new(MAIN_MENU_OPTIONS[2]),
        ListItem::new(MAIN_MENU_OPTIONS[3]),
        ListItem::new(MAIN_MENU_OPTIONS[4]),
    ];
    app.items = Vec::from(choices.clone());
    let new_list = List::new(choices)
        .block(Block::default().title("List").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol("->");
    let area = centered_rect(70, 60, frame.size());
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(new_list, area, &mut state);
    frame.render_widget(
        Paragraph::new("Press q to exit \n Press Enter to select \n Please select a Menu item\n")
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

pub fn render_command_menu<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, cmd: Command) {
    // Since we have the same menu for all 3, we'll just pass the command type into the next screen
    let choices = vec![
        ListItem::new(COMMAND_MENU_OPTIONS[0]),
        ListItem::new(COMMAND_MENU_OPTIONS[1]),
        ListItem::new(COMMAND_MENU_OPTIONS[2]),
        ListItem::new(COMMAND_MENU_OPTIONS[3]),
        ListItem::new(COMMAND_MENU_OPTIONS[4]),
        ListItem::new(COMMAND_MENU_OPTIONS[5]),
    ];
    let area = centered_rect(70, 60, frame.size());
    let new_list = List::new(choices)
        .block(
            Block::default()
                .title("Please choose a request type")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol("->");
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(new_list, area, &mut state);
    frame.render_widget(
        Paragraph::new("Press q to exit \n Press Enter to select \n Please select a Menu item\n")
            .block(
                Block::default()
                    .title("Build a new cURL command")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .alignment(Alignment::Center),
        frame.size(),
    );
    match cmd {
        Command::Curl => {
            app.current_screen = Screen::Command(Command::Curl);
        }
        Command::Wget => {
            app.current_screen = Screen::Command(Command::Wget);
        }
        Command::Custom => {
            app.current_screen = Screen::Command(Command::Custom);
        }
    }
}
pub fn render_keys_menu<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let choices = vec![
        ListItem::new("View my saved API Keys:\n \n"),
        ListItem::new("Add a new API Key to the database:\n \n"),
        ListItem::new("Remove an API Key from database:\n \n"),
        ListItem::new("View my saved cURL or wget commands\n \n"),
    ];
    let area = centered_rect(70, 50, frame.size());
    let new_list = List::new(choices)
        .block(Block::default().title("List").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol("->");
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(new_list, area, &mut state);
    frame.render_widget(
        Paragraph::new(
            "Create / Edit / Delete API Keys and tokens.\n
                    Press q to exit \n Press Enter to select \n Please select a Menu item\n",
        )
        .block(
            Block::default()
                .title("API Key Manager")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .alignment(Alignment::Center),
        frame.size(),
    )
}
// Helper func from ratatui exmaples
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
