use crate::app::InputMode;
use crate::app::{App, AppResult};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;
use tui_input::InputRequest;
/// Handles the key events and updates the state of [`App`].

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.input_mode {
        InputMode::Normal => {
            match key_event.code {
                // Exit application on `ESC` or `q`
                KeyCode::Esc | KeyCode::Char('q') => {
                    app.quit();
                }
                // Exit application on `Ctrl-C`
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.quit();
                    }
                }
                // Other handlers you could add here.
                KeyCode::Up => {
                    app.move_cursor_up();
                }
                KeyCode::Down => {
                    app.move_cursor_down();
                }
                KeyCode::Enter => {
                    app.select_item();
                }
                KeyCode::Char('e') => {
                    app.input_mode = InputMode::Editing;
                }
                KeyCode::Char('j') => {
                    app.move_cursor_down();
                }
                KeyCode::Char('k') => {
                    app.move_cursor_up();
                }
                KeyCode::Char('h') => {
                    app.go_back_screen();
                }

                _ => {}
            }
        }
        InputMode::Editing => match key_event.code {
            KeyCode::Enter => {
                app.messages.push(app.input.value().into());
                app.input.reset();
            }
            KeyCode::Char(c) => match app.input.handle(InputRequest::InsertChar(c)) {
                Some(_) => {}
                None => {}
            },
            KeyCode::Backspace => match app.input.handle(InputRequest::DeletePrevChar) {
                Some(_) => {}
                None => {}
            },

            KeyCode::Delete => match app.input.handle(InputRequest::DeleteNextChar) {
                Some(_) => {}
                None => {}
            },

            KeyCode::Left => match app.input.handle(InputRequest::GoToPrevChar) {
                Some(_) => {}
                None => {}
            },
            KeyCode::Right => match app.input.handle(tui_input::InputRequest::GoToNextChar) {
                Some(_) => {}
                None => {}
            },
            KeyCode::Esc => {
                app.input_mode = InputMode::Normal;
            }
            _ => {}
        },
    }
    Ok(())
}
