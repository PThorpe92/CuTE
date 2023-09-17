use crate::app::InputMode;
use crate::app::Screen;
use crate::app::CURL_MENU_OPTIONS;
use crate::app::HTTP_MENU_OPTIONS;
use crate::app::{App, Command};
use crate::curl::Curl;
use crate::wget::Wget;
use crate::{Request, DELETE, GET, PATCH, POST, PUT};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub static CURL: &str = "curl";
pub static WGET: &str = "wget";
pub static CUSTOM: &str = "custom";

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    match &app.current_screen.clone() {
        Screen::Home => {
            let new_list = app.current_screen.get_list();
            let area = centered_rect(70, 60, frame.size());
            let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
            app.state = Some(state.clone());
            app.state.as_mut().unwrap().select(Some(app.cursor));

            frame.set_cursor(0, app.cursor as u16);
            frame.render_stateful_widget(new_list, area, &mut state);
            frame.render_widget(menu_paragraph(), frame.size());
            match app.selected {
                Some(0) => {
                    app.goto_screen(Screen::Command(String::from(CURL)));
                    return;
                }
                Some(1) => {
                    app.goto_screen(Screen::Command(String::from(WGET)));
                    return;
                }
                Some(2) => {
                    app.goto_screen(Screen::Command(String::from(CUSTOM)));
                    return;
                }
                Some(3) => {
                    app.goto_screen(Screen::Keys);
                    return;
                }
                Some(_) => {}
                None => {}
            }
        }

        Screen::Command(cmd) => {
            let area = default_rect(frame.size());
            let new_list = app.current_screen.get_list();
            let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
            app.items = Vec::from(app.current_screen.get_opts());
            app.state = Some(state.clone());
            app.state.as_mut().unwrap().select(Some(app.cursor));
            frame.set_cursor(0, app.cursor as u16);
            frame.render_stateful_widget(new_list, area, &mut state);
            frame.render_widget(menu_paragraph(), frame.size());
            match cmd.as_str() {
                "curl" => app.command = Some(Command::Curl(Curl::new())),
                "wget" => app.command = Some(Command::Wget(Wget::new())),
                "custom" => app.command = Some(Command::Custom(Request::default())),
                _ => app.command = Some(Command::Custom(Request::default())),
            }
            match app.selected.clone() {
                Some(num) => {
                    app.command.as_mut().unwrap().set_method(String::from(GET));
                    app.goto_screen(Screen::CurlMenu(String::from(
                        HTTP_MENU_OPTIONS[num].clone(),
                    )));
                }
                None => {}
            }
        }
        Screen::Keys => {
            let area = default_rect(frame.size());
            let new_list = app.current_screen.get_list();
            let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
            if app.items.len() != 0 {
                app.items.clear();
            }
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

        Screen::CurlMenu(_) => {
            let area = default_rect(frame.size());
            let new_list = app.current_screen.get_list();
            let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
            if app.items.len() != 0 {
                app.items.clear();
            }
            app.items = Vec::from(app.current_screen.get_opts());
            app.state = Some(state.clone());
            app.state.as_mut().unwrap().select(Some(app.cursor));
            frame.set_cursor(0, app.cursor as u16);
            frame.render_stateful_widget(new_list, area, &mut state);
            frame.render_widget(menu_paragraph(), frame.size());
            match app.selected {
                // Add a URL
                Some(num) => {
                    app.goto_screen(Screen::InputMenu(num.clone()));
                }
                None => {}
            }
        }
        Screen::InputMenu(num) => {
            render_input_screen(app, frame, CURL_MENU_OPTIONS.get(*num).unwrap());
        }
        _ => {}
    }
}

/// Renders a screen we can grab input from, pass in the appropriate desination for the input
fn render_input_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, message: &str) {
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
        .split(frame.size());
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to submit."),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Line::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    frame.render_widget(help_message, chunks[0]);

    let width = chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor

    let scroll = app.input.visual_scroll(width as usize);
    let input = Paragraph::new(app.input.value())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .scroll((0, scroll as u16))
        .block(Block::default().borders(Borders::ALL).title("Input"));
    frame.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => frame.set_cursor(
            chunks[1].x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            chunks[1].y + 1,
        ),
    }
    if app.messages.len() > 0 {
        match message {
            "Add a URL\n \n" => {
                app.command
                    .as_mut()
                    .unwrap()
                    .set_url(app.messages[0].clone());
                app.input_mode = InputMode::Normal;
                app.current_screen = Screen::CurlMenu(String::new())
            }
            "Add Headers\n \n" => app
                .command
                .as_mut()
                .unwrap()
                .set_url(app.messages[0].clone()),
            "Add Authentication\n \n" => app
                .command
                .as_mut()
                .unwrap()
                .set_url(app.messages[0].clone()),
            &_ => {}
        }
    }
}

fn menu_paragraph() -> Paragraph<'static> {
    Paragraph::new("\nPress q to exit \n Press Enter to select \n Please select a Menu item\n")
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

fn default_rect(r: Rect) -> Rect {
    centered_rect(70, 60, r)
}
