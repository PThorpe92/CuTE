use arboard::Clipboard;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use tui_input::InputRequest;

use crate::app::InputMode;
use crate::app::{App, AppResult};
use crate::display::inputopt::InputOpt;
use crate::screens::screen::Screen;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.input_mode {
        InputMode::Normal => {
            match key_event.kind {
                KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Char('q') => {
                            app.quit();
                        }
                        // Exit application on `Ctrl-C`
                        KeyCode::Char('c') | KeyCode::Char('C') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                app.quit();
                            }
                        }
                        KeyCode::Esc => {
                            app.go_back_screen(); // Escape Should Bring You Back
                            if !app.input.value().is_empty() {
                                app.input.reset(); // If we leave the page, we should clear the input buffer
                            }
                        }
                        KeyCode::Up => {
                            app.move_cursor_up();
                        }
                        KeyCode::Down => {
                            app.move_cursor_down();
                        }
                        KeyCode::Enter => {
                            app.select_item();
                        }
                        KeyCode::Char('a') => {
                            if app.current_screen == Screen::SavedKeys {
                                app.goto_screen(&Screen::InputMenu(InputOpt::ApiKey));
                            }
                        }
                        KeyCode::Char('i') => match &app.current_screen {
                            Screen::InputMenu(_) => {
                                app.input_mode = InputMode::Editing;
                            }
                            Screen::RequestMenu(opt) if opt.is_some() => {
                                if !opt.as_ref().unwrap().is_error() {
                                    app.input_mode = InputMode::Editing;
                                }
                            }
                            Screen::SavedCollections(opt) if opt.is_some() => {
                                if !opt.as_ref().unwrap().is_error() {
                                    app.input_mode = InputMode::Editing;
                                }
                            }
                            Screen::RequestBodyInput => {
                                app.input_mode = InputMode::Editing;
                            }
                            _ => {}
                        },
                        KeyCode::Char('j') => {
                            app.move_cursor_down();
                        }
                        KeyCode::Char('k') => {
                            app.move_cursor_up();
                        }
                        KeyCode::Char('h') => {
                            // if we are going back to method scree, clear everything
                            app.go_back_screen();
                        }
                        KeyCode::Char('b') => {
                            app.go_back_screen();
                        }
                        _ => {}
                    }
                }
                KeyEventKind::Release => {
                    // Release Key Event Bindings
                }
                KeyEventKind::Repeat => {
                    // Repeat Key Event Bindings
                }
            }
        }
        InputMode::Editing => match key_event.kind {
            KeyEventKind::Press => match key_event.code {
                KeyCode::Enter => {
                    app.messages.push(app.input.value().trim().to_string());
                    app.input_mode = InputMode::Normal;
                    app.input.reset();
                }
                KeyCode::Char(c) => {
                    if key_event.modifiers == KeyModifiers::CONTROL && c == 'v' {
                        if let Ok(s) = Clipboard::new().and_then(|mut ctx| ctx.get_text()) {
                            for ch in s.trim().chars() {
                                app.input.handle(InputRequest::InsertChar(ch));
                            }
                        }
                    } else if app.input.handle(InputRequest::InsertChar(c)).is_some() {
                    }
                }
                KeyCode::Backspace => {
                    if app.input.handle(InputRequest::DeletePrevChar).is_some() {}
                }
                KeyCode::Delete => if app.input.handle(InputRequest::DeleteNextChar).is_some() {},
                // if ctrl + left or ctrl + right is pressed, move one word at a time in either
                // direction respectively
                KeyCode::Left => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.input.handle(InputRequest::GoToPrevWord);
                    } else if app.input.handle(InputRequest::GoToPrevChar).is_some() {
                    }
                }
                KeyCode::Right => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.input.handle(InputRequest::GoToNextWord);
                    } else if app
                        .input
                        .handle(tui_input::InputRequest::GoToNextChar)
                        .is_some()
                    {
                    }
                }
                KeyCode::Esc => {
                    app.input_mode = InputMode::Normal;
                }
                _ => {}
            },
            KeyEventKind::Release => {}
            KeyEventKind::Repeat => {}
        },
    }
    Ok(())
}
