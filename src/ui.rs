use crate::app::Screen;
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::app::{App, Command};

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
        ListItem::new("Build and run a new cURL command\n  \n"),
        ListItem::new("Build and run a new wget command\n  \n"),
        ListItem::new("Build/send new custom HTTP request\n  \n"),
        ListItem::new("View my stored API keys\n  \n"),
        ListItem::new("View or execute my saved commands\n  \n"),
    ];
    app.items = Vec::from(choices.clone());
    let new_list = List::new(choices)
        .block(Block::default().title("List").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol("->");
    let area = Rect::new(0, 0, 40, 10);
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
    match cmd {
        Command::Curl => {
            let choices = vec![
                ListItem::new("Choose an HTTP method:"),
                ListItem::new("GET"),
                ListItem::new("POST"),
                ListItem::new("PUT"),
                ListItem::new("DELETE"),
                ListItem::new("PATCH"),
                ListItem::new("HEAD"),
                ListItem::new("OPTIONS"),
            ];
            let area = Rect::new(0, 0, 40, 10);
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
                    "Press q to exit \n Press Enter to select \n Please select a Menu item\n",
                )
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
            )
        }
        Command::Wget => {
            let choices = vec![
                ListItem::new("Choose an HTTP method:"),
                ListItem::new("GET"),
                ListItem::new("POST"),
                ListItem::new("PUT"),
                ListItem::new("DELETE"),
                ListItem::new("PATCH"),
                ListItem::new("HEAD"),
                ListItem::new("OPTIONS"),
            ];
            let area = Rect::new(0, 0, 40, 10);
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
                    "Press q to exit \n Press Enter to select \n Please select a Menu item\n",
                )
                .block(
                    Block::default()
                        .title("Build a new wget command")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                .alignment(Alignment::Center),
                frame.size(),
            )
        }
        Command::Custom => {
            let choices = vec![
                ListItem::new("Choose an HTTP method:"),
                ListItem::new("GET"),
                ListItem::new("POST"),
                ListItem::new("PUT"),
                ListItem::new("DELETE"),
                ListItem::new("PATCH"),
                ListItem::new("HEAD"),
                ListItem::new("OPTIONS"),
            ];
            let area = Rect::new(0, 0, 40, 10);
            let new_list = List::new(choices)
                .block(
                    Block::default()
                        .title("Build a custom HTTP request")
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
                Paragraph::new(
                    "Please select a method type for your custom HTTP request.\n
                    Press q to exit \n Press Enter to select \n Please select a Menu item\n",
                )
                .block(
                    Block::default()
                        .title("Custom HTTP Request Builder")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                .alignment(Alignment::Center),
                frame.size(),
            )
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
    let area = Rect::new(0, 0, 40, 10);
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
