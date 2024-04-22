use super::input::input_screen::handle_default_input_screen;
use super::render::render_header_paragraph;
use super::{centered_rect, error_alert_box, Screen, ScreenArea};
use crate::app::App;
use crate::display::inputopt::InputOpt;
use crate::display::menuopts::{CMD_MENU_OPTIONS, SAVED_COMMANDS_PARAGRAPH, SAVED_COMMANDS_TITLE};
use tui::prelude::{Constraint, Direction, Layout, Margin};
use tui::style::{Color, Modifier, Style};
use tui::text::{Line, Span};

use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use tui::Frame;

pub fn handle_saved_commands_screen(
    app: &mut App,
    frame: &mut Frame<'_>,
    coll: Option<i32>,
    opt: Option<InputOpt>,
) {
    let commands = app.db.as_ref().get_commands(coll).unwrap_or_default();
    let items = Some(
        commands
            .iter()
            .map(|x| {
                format!(
                    "Request: {}  |  Collection: {:?}",
                    x.label.clone().unwrap_or(String::from("No label")),
                    match x.collection_name.clone() {
                        Some(name) => name,
                        None => "No Collection".to_string(),
                    }
                )
            })
            .collect::<Vec<String>>(),
    );
    let menu_options = app.current_screen.get_list(items);
    let area = centered_rect(frame.size(), ScreenArea::Center);
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.render_stateful_widget(menu_options, area, &mut state);
    let (paragraph, title) = (&SAVED_COMMANDS_PARAGRAPH, &SAVED_COMMANDS_TITLE);
    frame.render_widget(
        render_header_paragraph(paragraph, title, app.config.get_style()),
        frame.size(),
    );

    match opt {
        Some(InputOpt::AlertMessage(msg)) | Some(InputOpt::RequestError(msg)) => {
            error_alert_box(frame, &msg);
        }
        _ => {}
    }
    if let Some(selected) = app.selected {
        let cmd = commands.get(selected);
        if let Some(cmd) = cmd {
            app.goto_screen(&Screen::CmdMenu {
                id: cmd.get_id(),
                opt: None,
            });
        } else {
            app.goto_screen(&Screen::SavedCommands {
                id: None,
                opt: Some(InputOpt::RequestError("No commands found".to_string())),
            });
        }
    }
}

pub fn handle_alert_menu(app: &mut App, frame: &mut Frame<'_>, cmd: i32, opt: Option<InputOpt>) {
    if let Some(opt) = opt {
        handle_default_input_screen(app, frame, opt);
    }
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .horizontal_margin(5)
        .split(frame.size());
    let options_box = layout[1].inner(&Margin {
        vertical: 1,
        horizontal: 15,
    });
    let mut list_state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.state = Some(list_state.clone());
    let items: Vec<ListItem> = CMD_MENU_OPTIONS
        .iter()
        .map(|option| ListItem::new(*option))
        .collect();
    let list = List::new(items)
        .block(Block::default())
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    let cmd_str = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(layout[1])[1];
    if let Ok(command) = app.db.as_ref().get_command_by_id(cmd) {
        let collection_name = match app
            .db
            .as_ref()
            .get_collection_by_id(command.collection_id.unwrap_or(0))
        {
            Ok(collection) => collection.name,
            Err(_) => "No Collection".to_string(),
        };
        let alert_text = vec![
            Line::raw("\n"),
            Line::default().spans(vec![
                Span::styled("Label: ", Style::default().fg(Color::LightGreen)),
                Span::styled(
                    command.label.clone().unwrap_or("None".to_string()),
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::default().spans(vec![
                Span::styled("Description: ", Style::default().fg(Color::LightGreen)),
                Span::styled(
                    command.description.clone().unwrap_or("None".to_string()),
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::default().spans(vec![
                Span::styled("Collection: ", Style::default().fg(Color::LightGreen)),
                Span::styled(collection_name, Style::default().fg(Color::White)),
            ]),
            Line::default().spans(vec![
                Span::styled("ID: ", Style::default().fg(Color::LightGreen)),
                Span::styled(command.id.to_string(), Style::default().fg(Color::White)),
            ]),
        ];
        let alert_text = List::new(alert_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Command Details"),
            )
            .style(Style::default().fg(Color::Blue))
            .highlight_style(Style::default().fg(Color::LightGreen));
        frame.render_stateful_widget(list, options_box, &mut list_state);
        frame.render_widget(alert_text, layout[0]);
        let header = Block::default().borders(Borders::ALL).title("* Request *");
        frame.render_widget(header, layout[0]);
        let paragraph = Paragraph::new(command.get_command())
            .block(Block::default().borders(Borders::ALL).title("* Command *"))
            .alignment(tui::layout::Alignment::Center)
            .centered()
            .wrap(Wrap::default());
        frame.render_widget(paragraph, cmd_str);
        match app.selected {
            // execute saved command
            Some(0) => {
                app.execute_saved_command(command.get_curl_json());
                app.goto_screen(&Screen::Response(app.response.clone().unwrap()));
            }
            // add a label
            Some(1) => {
                app.goto_screen(&Screen::CmdMenu {
                    id: cmd,
                    opt: Some(InputOpt::CmdLabel(cmd)),
                });
            }
            // add a description
            Some(2) => {
                app.goto_screen(&Screen::CmdMenu {
                    id: cmd,
                    opt: Some(InputOpt::CmdDescription(cmd)),
                });
            }
            // delete item
            Some(3) => {
                if let Err(e) = app.delete_item(command.get_id()) {
                    app.goto_screen(&Screen::SavedCommands {
                        id: None,
                        opt: Some(InputOpt::RequestError(format!("Error: {}", e))),
                    });
                } else {
                    app.goto_screen(&Screen::SavedCommands {
                        id: None,
                        opt: Some(InputOpt::AlertMessage(
                            "Successfully deleted command".to_string(),
                        )),
                    });
                }
            }
            // copy to clipboard
            Some(4) => {
                if let Err(e) = app.copy_to_clipboard(command.get_command()) {
                    app.goto_screen(&Screen::Error(e.to_string()));
                }
                app.goto_screen(&Screen::SavedCommands {
                    id: None,
                    opt: Some(InputOpt::AlertMessage(
                        "CLI Command copied to clipboard".to_string(),
                    )),
                });
            }
            // cancel
            Some(5) => {
                app.goto_screen(&Screen::SavedCommands {
                    id: None,
                    opt: None,
                });
            }
            _ => {}
        }
    }
}
