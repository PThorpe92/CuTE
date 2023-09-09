use crate::app::Screen;
use crate::app::{App, Command};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    match &app.current_screen {
        Screen::Home => {
            render_home(app, frame);
            match app.selected {
                Some(0) => {
                    app.current_screen = Screen::Command(Command::Curl);
                    return;
                }
                Some(1) => {
                    app.current_screen = Screen::Command(Command::Wget);
                    return;
                }
                Some(2) => {
                    app.current_screen = Screen::Command(Command::Custom);
                    return;
                }
                Some(3) => {
                    app.current_screen = Screen::Keys;
                    return;
                }
                Some(_) => {}
                None => {}
            }
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
    let new_list = app.current_screen.get_list();
    let area = centered_rect(70, 60, frame.size());
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));

    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(new_list, area, &mut state);
    frame.render_widget(menu_paragraph(), frame.size());
}

pub fn render_command_menu<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, cmd: Command) {
    let area = centered_rect(70, 60, frame.size());
    let new_list = app.current_screen.get_list();
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.items = Vec::from(app.current_screen.get_opts());
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(new_list, area, &mut state);
    frame.render_widget(menu_paragraph(), frame.size());
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
    let area = centered_rect(70, 50, frame.size());
    let new_list = app.current_screen.get_list();
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.items = Vec::from(app.current_screen.get_opts());
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

fn menu_paragraph() -> Paragraph<'static> {
    Paragraph::new("Press q to exit \n Press Enter to select \n Please select a Menu item\n")
        .block(
            Block::default()
                .title("cURL-TUI")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .alignment(Alignment::Center)
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
