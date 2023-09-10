use crate::app::Screen;
use crate::app::{App, Command};
use crate::curl::{Curl, CurlFlag, CurlFlagType};
use crate::wget::Wget;
use crate::{Request, DELETE, GET, PATCH, POST, PUT};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, ListState, Paragraph},
    Frame,
};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    match &app.current_screen.clone() {
        Screen::Home => {
            render_home(app, frame);
            match app.selected {
                Some(0) => {
                    app.goto_screen(Screen::Command(Command::Curl));
                    return;
                }
                Some(1) => {
                    app.goto_screen(Screen::Command(Command::Wget));
                    return;
                }
                Some(2) => {
                    app.goto_screen(Screen::Command(Command::Custom));
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
            render_command_menu(app, frame, cmd.clone());
            app.command = Some(cmd.clone());
            match app.selected.clone() {
                Some(1) => {
                    // GET
                    match *cmd {
                        Command::Curl => {
                            let mut curl = Curl::new();
                            curl.add_flag(CurlFlag::new(None, CurlFlagType::Method), Some(GET));
                            app.goto_screen(Screen::CurlMenu(curl));
                            return;
                        }
                        Command::Wget => {
                            let wget = Wget::new(GET);
                            app.goto_screen(Screen::WgetMenu(wget));
                            return;
                        }
                        Command::Custom => {
                            app.goto_screen(Screen::CustomMenu(Request::default()));
                            return;
                        }
                    }
                }
                // POST
                Some(2) => match *cmd {
                    Command::Curl => {
                        let mut curl = Curl::new();
                        curl.add_flag(CurlFlag::new(None, CurlFlagType::Method), Some(POST));
                        app.goto_screen(Screen::CurlMenu(curl));
                        return;
                    }
                    Command::Wget => {
                        let wget = Wget::new(POST);
                        app.goto_screen(Screen::WgetMenu(wget));
                        return;
                    }
                    Command::Custom => {
                        app.goto_screen(Screen::CustomMenu(Request::default()));
                        return;
                    }
                },
                // PUT
                Some(3) => match *cmd {
                    Command::Curl => {
                        let mut curl = Curl::new();
                        curl.add_flag(CurlFlag::new(None, CurlFlagType::Method), Some(PUT));
                        app.goto_screen(Screen::CurlMenu(curl));
                        return;
                    }
                    Command::Wget => {
                        let wget = Wget::new(PUT);
                        app.goto_screen(Screen::WgetMenu(wget));
                        return;
                    }
                    Command::Custom => {
                        app.goto_screen(Screen::CustomMenu(Request::default()));
                        return;
                    }
                },
                // DELETE
                Some(4) => match *cmd {
                    Command::Curl => {
                        let mut curl = Curl::new();
                        curl.add_flag(CurlFlag::new(None, CurlFlagType::Method), Some(DELETE));
                        app.goto_screen(Screen::CurlMenu(curl));
                        return;
                    }
                    Command::Wget => {
                        let wget = Wget::new(DELETE);
                        app.goto_screen(Screen::WgetMenu(wget));
                        return;
                    }
                    Command::Custom => {
                        app.goto_screen(Screen::CustomMenu(Request::default()));
                        return;
                    }
                },
                // PATCH
                Some(5) => match *cmd {
                    Command::Curl => {
                        let mut curl = Curl::new();
                        curl.add_flag(CurlFlag::new(None, CurlFlagType::Method), Some(PATCH));
                        app.goto_screen(Screen::CurlMenu(curl));
                        return;
                    }
                    Command::Wget => {
                        let wget = Wget::new(PATCH);
                        app.goto_screen(Screen::WgetMenu(wget));
                        return;
                    }
                    Command::Custom => {
                        app.goto_screen(Screen::CustomMenu(Request::default()));
                        return;
                    }
                },
                Some(_) => {}
                _ => {}
            }
        }
        Screen::Keys => {
            render_keys_menu(app, frame);
        }
        Screen::CurlMenu(_) => {
            render_curl_menu(app, frame, Command::Curl);
            match app.selected {
                Some(0) => {
                    app.goto_screen(Screen::Command(Command::Curl));
                    return;
                }
                Some(1) => {
                    app.goto_screen(Screen::Command(Command::Wget));
                    return;
                }
                Some(2) => {
                    app.goto_screen(Screen::Command(Command::Custom));
                    return;
                }
                Some(_) => {}
                None => {}
            }
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
    let area = default_rect(frame.size());
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
            app.current_screen = Screen::CurlMenu(Curl::new());
        }
        Command::Wget => {
            app.current_screen = Screen::Command(Command::Wget);
        }
        Command::Custom => {
            app.current_screen = Screen::Command(Command::Custom);
        }
    }
}

pub fn render_curl_menu<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, cmd: Command) {
    let area = default_rect(frame.size());
    let new_list = app.current_screen.get_list();

    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.items = Vec::from(app.current_screen.get_opts());
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(new_list, area, &mut state);
    frame.render_widget(menu_paragraph(), frame.size());
}

pub fn render_keys_menu<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let area = default_rect(frame.size());
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
