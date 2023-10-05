use crate::display::menuopts::{
    API_KEY_PARAGRAPH, API_KEY_TITLE, AUTH_MENU_TITLE, DEFAULT_MENU_PARAGRAPH, DEFAULT_MENU_TITLE,
    DOWNLOAD_MENU_TITLE, ERROR_MENU_TITLE, INPUT_MENU_TITLE, SAVED_COMMANDS_TITLE,
    SUCCESS_MENU_TITLE, VIEW_BODY_TITLE,
};
use crate::display::DisplayOpts;
use crate::screens::input::input::handle_default_input_screen;

use super::auth::handle_authentication_screen;
use super::debug::handle_debug_screen;
use super::downloads::handle_downloads_screen;
use super::home::handle_home_screen;
use super::keys::{handle_api_key_screen, handle_view_saved_keys};
use super::method::handle_method_select_screen;
use super::request::handle_request_menu_screen;
use super::response::handle_response_screen;
use super::saved_commands::handle_saved_commands_screen;
use crate::{app::App, display::menuopts::SAVED_COMMANDS_PARAGRAPH};

use tui::widgets::ListState;
use tui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use super::{centered_rect, small_rect, Screen};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    //
    // If we already have a response, we render that instead of the opts
    if app.response.is_none() {
        //
        // Render Display Options *******************************************
        // This is the box of options the user has selected so far in their current
        // command. This is rendered on the bottom of the screen. Each time we change
        // app.current_screen, this function is called so we check for any display options
        // that were added to app.opts in the previous screen and add them here.
        let mut display_opts = String::new();
        app.opts.iter().for_each(|opt| match opt {
            DisplayOpts::Verbose => {
                display_opts.push_str("- Verbose\n");
            }
            DisplayOpts::URL(url) => {
                let url_str = format!("- URL: {}\n", &url);
                display_opts.push_str(url_str.as_str());
            }
            DisplayOpts::RecDownload(num) => {
                let rec_str = format!("- Recursive Download depth: {}\n", num);
                display_opts.push_str(rec_str.as_str());
            }
            DisplayOpts::SaveCommand => {
                display_opts.push_str("- Command will be saved\n");
            }
            DisplayOpts::Auth(auth) => {
                let auth_str = format!("- Auth: {}\n", auth);
                display_opts.push_str(auth_str.as_str());
            }
            _ => {}
        });
        let final_opts = display_opts.clone();
        let opts = Paragraph::new(final_opts.as_str())
            .block(
                Block::default()
                    .title("Options")
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Gray))
            .alignment(Alignment::Left);
        let area = small_rect(frame.size());
        frame.render_widget(opts, area);
        // ******************************************************************
    } else {
        let area = small_rect(frame.size());
        let response = app.response.clone().unwrap();
        let paragraph = Paragraph::new(Text::from(response.as_str()))
            .style(Style::default().fg(Color::Yellow).bg(Color::Black))
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, area);
    }
    // We pass this off where we match on the current screen and render what we need to
    handle_screen(app, frame, app.current_screen.clone());
}

pub fn handle_screen_defaults<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let menu_options = app.current_screen.get_list();
    let area = centered_rect(70, 60, frame.size());

    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));

    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(menu_options, area, &mut state);
    let (paragraph, title) = match app.current_screen {
        Screen::Home => (&DEFAULT_MENU_PARAGRAPH, &DEFAULT_MENU_TITLE),
        Screen::SavedCommands => (&SAVED_COMMANDS_PARAGRAPH, &SAVED_COMMANDS_TITLE),
        Screen::Response(_) => (&DEFAULT_MENU_PARAGRAPH, &DEFAULT_MENU_TITLE),
        Screen::InputMenu(_) => (&DEFAULT_MENU_PARAGRAPH, &INPUT_MENU_TITLE),
        Screen::Authentication => (&DEFAULT_MENU_PARAGRAPH, &AUTH_MENU_TITLE),
        Screen::Success => (&DEFAULT_MENU_PARAGRAPH, &SUCCESS_MENU_TITLE),
        Screen::Error(_) => (&DEFAULT_MENU_PARAGRAPH, &ERROR_MENU_TITLE),
        Screen::ViewBody => (&DEFAULT_MENU_PARAGRAPH, &VIEW_BODY_TITLE),
        Screen::Downloads => (&DEFAULT_MENU_PARAGRAPH, &DOWNLOAD_MENU_TITLE),
        Screen::SavedKeys => (&API_KEY_PARAGRAPH, &API_KEY_TITLE),
        Screen::HeaderAddRemove => (&DEFAULT_MENU_PARAGRAPH, &DEFAULT_MENU_TITLE),
        _ => (&DEFAULT_MENU_PARAGRAPH, &DEFAULT_MENU_TITLE),
    };
    frame.render_widget(render_header_paragraph(paragraph, title), frame.size());
}

pub fn handle_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, screen: Screen) {
    match screen {
        Screen::Home => handle_home_screen(app, frame),
        Screen::Method => handle_method_select_screen(app, frame),
        Screen::ViewBody => {
            let area = small_rect(frame.size());
            let response = app.response.clone().unwrap();
            let paragraph = Paragraph::new(Text::from(response.as_str()))
                .style(Style::default().fg(Color::Yellow).bg(Color::Black))
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, area);
        }
        Screen::Downloads => handle_downloads_screen(app, frame),
        Screen::RequestMenu(_) => handle_request_menu_screen(app, frame),
        // KEYS SCREEN **********************************************************
        Screen::KeysMenu => {
            handle_api_key_screen(app, frame);
        }
        // AUTHENTICATION SCREEN ************************************************
        Screen::Authentication => {
            handle_authentication_screen(app, frame);
        }
        // SUCESSS SCREEN *******************************************************
        Screen::Success => handle_screen_defaults(app, frame),
        // INPUT MENU SCREEN ****************************************************
        Screen::InputMenu(opt) => {
            handle_default_input_screen(app, frame, opt.clone());
        }
        // RESPONSE SCREEN ******************************************************
        Screen::Response(resp) => {
            app.set_response(resp.clone());
            handle_response_screen(app, frame, resp.to_string());
        }
        // DEBUG MENU ***********************************************************
        Screen::Debug => {
            handle_debug_screen(app, frame);
        }
        Screen::SavedCommands => {
            handle_saved_commands_screen(app, frame);
        }
        Screen::Error(e) => {
            handle_response_screen(app, frame, e.to_string());
        }
        Screen::SavedKeys => {
            handle_view_saved_keys(app, frame);
        }
        _ => {}
    }
}
pub fn render_header_paragraph(para: &'static str, title: &'static str) -> Paragraph<'static> {
    Paragraph::new(para)
        .block(
            Block::default()
                .title(title)
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .alignment(Alignment::Center)
}
