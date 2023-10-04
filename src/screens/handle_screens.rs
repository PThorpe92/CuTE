use tui::backend::Backend;
use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::{ListState, Paragraph};
use tui::Frame;

use crate::app::App;
use crate::display::inputopt::InputOpt;
use crate::request::cmdtype::CmdType;
use crate::request::command::Command;
use crate::request::curl::Curl;
use crate::request::wget::Wget;
use crate::screens::input::input::handle_default_input_screen;
use crate::ui::render::{render_header_paragraph, HOME_MENU_PARAGRAPH, HOME_MENU_TITLE};
use crate::ui::widgets::boxes::{centered_rect, default_rect, small_rect};

use super::auth::handle_authentication_screen;
use super::debug::handle_debug_screen;
use super::home::handle_home_screen;
use super::input::url::handle_url_input_screen;
use super::keys::handle_api_key_screen;
use super::method::handle_method_select_screen;
use super::request::handle_request_menu_screen;
use super::response::handle_response_screen;
use super::saved_commands::handle_saved_commands_screen;
use super::success::handle_success_screen;
use super::Screen;

pub fn handle_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, screen: Screen) {
    match screen {
        Screen::Home => {
            handle_home_screen(app, frame);
            let new_list = app.current_screen.get_list();
            let area = centered_rect(70, 60, frame.size());
            let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
            app.state = Some(state.clone());
            app.state.as_mut().unwrap().select(Some(app.cursor));
            frame.set_cursor(0, app.cursor as u16);
            frame.render_stateful_widget(new_list, area, &mut state);
            frame.render_widget(
                render_header_paragraph(HOME_MENU_PARAGRAPH, HOME_MENU_TITLE),
                frame.size(),
            );
            if let Some(num) = app.selected {
                match num {
                    0 => {
                        app.command = Some(Command::Curl(Curl::new()));
                        app.goto_screen(Screen::Method);
                    }
                    1 => {
                        app.command = Some(Command::Wget(Wget::new()));
                        app.goto_screen(Screen::Downloads);
                    }
                    2 => app.goto_screen(Screen::Keys),
                    3 => app.goto_screen(Screen::Commands),
                    _ => {}
                }
            }
        }
        Screen::Method => {
            handle_method_select_screen(app, frame);
        }
        Screen::ViewBody => {
            let area = small_rect(frame.size());
            let response = app.response.clone().unwrap();
            let paragraph = Paragraph::new(Text::from(response.as_str()))
                .style(Style::default().fg(Color::Yellow).bg(Color::Black))
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, area);
        }
        Screen::Downloads => {
            app.command = Some(Command::Wget(Wget::new()));
            let area = default_rect(frame.size());
            let list = app.current_screen.get_list();
            let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
            app.items = app.current_screen.get_opts();
            app.state = Some(state.clone());
            app.state.as_mut().unwrap().select(Some(app.cursor));
            frame.set_cursor(0, app.cursor as u16);
            frame.render_stateful_widget(list, area, &mut state);
            frame.render_widget(
                render_header_paragraph(HOME_MENU_PARAGRAPH, HOME_MENU_TITLE),
                frame.size(),
            );

            match app.selected {
                // Setting Recursion level
                Some(0) => {
                    app.goto_screen(Screen::InputMenu(InputOpt::RecursiveDownload));
                    app.selected = None;
                }
                // Add URL of download
                Some(1) => {
                    app.goto_screen(Screen::InputMenu(InputOpt::URL(CmdType::Wget)));
                    app.selected = None;
                }
                // Add file name for output/download
                Some(2) => {
                    app.goto_screen(Screen::InputMenu(InputOpt::Output));
                    app.selected = None;
                    // Execute command
                }
                Some(3) => match app.execute_command() {
                    Ok(_) => {
                        if let Some(response) = app.command.as_ref().unwrap().get_response() {
                            app.response = Some(response.clone());
                            app.goto_screen(Screen::Response(response));
                        }
                    }
                    Err(e) => {
                        app.goto_screen(Screen::Error(e));
                    }
                },
                _ => {}
            };
        }
        Screen::RequestMenu(_) => {
            handle_request_menu_screen(app, frame);
        }
        // KEYS SCREEN **********************************************************
        Screen::Keys => {
            handle_api_key_screen(app, frame);
        }
        // AUTHENTICATION SCREEN ************************************************
        Screen::Authentication => {
            handle_authentication_screen(app, frame);
        }
        // SUCESSS SCREEN *******************************************************
        Screen::Success => {
            handle_success_screen(app, frame);
        }
        // INPUT MENU SCREEN ****************************************************
        Screen::InputMenu(opt) => {
            handle_default_input_screen(app, frame, opt.clone());
        }
        // RESPONSE SCREEN ******************************************************
        Screen::Response(resp) => {
            handle_response_screen(app, frame, resp.to_string());
        }
        // DEBUG MENU ***********************************************************
        Screen::Debug => {
            handle_debug_screen(app, frame);
        }
        // URL INPUT SCREEN *****************************************************
        Screen::URLInput => {
            handle_url_input_screen(app, frame);
        }
        Screen::Commands => {
            handle_saved_commands_screen(app, frame);
        }
        Screen::Error(e) => {
            handle_response_screen(app, frame, e.to_string());
        }
        _ => {}
    }
}
