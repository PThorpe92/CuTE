use super::input::input_screen::handle_default_input_screen;
use super::render::handle_screen_defaults;
use super::{centered_rect, error_alert_box, Screen, ScreenArea};
use crate::app::App;
use crate::display::inputopt::InputOpt;
use crate::display::menuopts::KEY_MENU_OPTIONS;
use tui::prelude::{Constraint, Direction, Layout, Margin};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph};
use tui::Frame;

pub fn handle_saved_keys_screen(app: &mut App, frame: &mut Frame<'_>, opt: Option<InputOpt>) {
    handle_screen_defaults(app, frame);
    match opt {
        Some(InputOpt::AlertMessage(msg)) | Some(InputOpt::RequestError(msg)) => {
            error_alert_box(frame, &msg);
        }
        Some(opt) => {
            handle_default_input_screen(app, frame, opt.clone());
        }
        None => {
            if app.items.is_empty() {
                let paragraph = Paragraph::new("No Keys Found. Press 'a' to add a new key.").block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Double)
                        .border_style(Style::default().fg(Color::Red)),
                );
                frame.render_widget(paragraph, centered_rect(frame.size(), ScreenArea::Center))
            } else {
                let paragraph =
                    Paragraph::new("Press 'a' to add a new key").style(Style::default());
                frame.render_widget(paragraph, centered_rect(frame.size(), ScreenArea::Top));
            }
        }
    };
    // if we select a key, open options
    if let Some(cmd) = app.selected {
        app.goto_screen(&Screen::KeysMenu(cmd));
    }
}

pub fn handle_key_menu(app: &mut App, frame: &mut Frame<'_>, cmd: usize) {
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
    // Render the alert box
    let alert_box = layout[1];
    let alert_text_chunk = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black).fg(Color::LightRed))
        .title("My API Keys");
    let options_box = layout[1].inner(&Margin {
        vertical: 1,
        horizontal: 1,
    });
    let mut list_state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.state = Some(list_state.clone());
    let items: Vec<ListItem> = KEY_MENU_OPTIONS
        .iter()
        .map(|option| ListItem::new(*option))
        .collect();
    let list = List::new(items.clone())
        .block(Block::default())
        .highlight_style(
            Style::default()
                .bg(Color::White)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    let cmd_str = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(alert_box)[1];
    let show_cmds = app.db.as_ref().get_keys().unwrap();
    let selected = show_cmds.get(cmd).unwrap().clone();
    let paragraph = Paragraph::new(format!("{:?}", selected))
        .block(Block::default().borders(Borders::ALL).title("Selected Key"))
        .alignment(tui::layout::Alignment::Center);
    frame.render_widget(paragraph, cmd_str);
    frame.render_widget(alert_text_chunk, alert_box);
    frame.render_stateful_widget(list, options_box, &mut list_state);
    match app.selected {
        // Add/Edit Label
        Some(0) => {
            app.goto_screen(&Screen::SavedKeys(Some(
                crate::display::inputopt::InputOpt::KeyLabel(selected.get_id()),
            )));
        }
        // delete item
        Some(1) => {
            if let Err(e) = app.delete_item(selected.get_id()) {
                app.goto_screen(&Screen::SavedKeys(Some(InputOpt::RequestError(format!(
                    "Error: {e}"
                )))));
            } else {
                app.goto_screen(&Screen::SavedKeys(Some(InputOpt::AlertMessage(
                    String::from("Key Deleted"),
                ))));
            }
        }
        // copy to clipboard
        Some(2) => match app.copy_to_clipboard(selected.get_key()) {
            Err(e) => app.goto_screen(&Screen::SavedKeys(Some(InputOpt::RequestError(format!(
                "Error: {e}"
            ))))),

            Ok(_) => app.goto_screen(&Screen::SavedKeys(Some(InputOpt::AlertMessage(
                String::from("Key copied to clipboard"),
            )))),
        },
        // cancel
        Some(3) => {
            app.goto_screen(&Screen::SavedKeys(None));
        }
        _ => {}
    }
}
