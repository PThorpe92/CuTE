use crate::display::inputopt::InputOpt;
use crate::display::menuopts::{
    API_KEY_PARAGRAPH, API_KEY_TITLE, AUTH_MENU_TITLE, DEFAULT_MENU_PARAGRAPH, DEFAULT_MENU_TITLE,
    DOWNLOAD_MENU_TITLE, ERROR_MENU_TITLE, INPUT_MENU_TITLE, SAVED_COMMANDS_TITLE,
    SUCCESS_MENU_TITLE, VIEW_BODY_TITLE,
};
use crate::display::AppOptions;
use crate::screens::input::input::handle_default_input_screen;

use super::auth::handle_authentication_screen;
use super::downloads::handle_downloads_screen;
use super::home::handle_home_screen;
use super::input::request_body_input::handle_req_body_input_screen;
use super::method::handle_method_select_screen;
use super::more_flags::handle_more_flags_screen;
use super::request::handle_request_menu_screen;
use super::response::handle_response_screen;
use super::saved_commands::{handle_alert_menu, handle_saved_commands_screen};
use super::saved_keys::{handle_key_menu, handle_saved_keys_screen};
use crate::screens::error::handle_error_screen;
use crate::{app::App, display::menuopts::SAVED_COMMANDS_PARAGRAPH};
use tui::style::Stylize;
use tui::text::Line;
use tui::widgets::ListState;
use tui::widgets::{Block, Borders};
use tui::{
    backend::Backend,
    layout::Alignment,
    style::Style,
    text::Text,
    widgets::{BorderType, Paragraph},
    Frame,
};

use super::{centered_rect, default_rect, small_rect, Screen};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    if app.response.is_none() {
        // Render Display Options *******************************************
        // This is the box of options the user has selected so far in their current
        // command. This is rendered on the bottom of the screen. Each time we change
        // app.current_screen, this function is called so we check for any display options
        // that were added to app.opts in the previous screen and add them here.
        if app.current_screen == Screen::Home {
            // the home screen renders the ascii art logo
            let logo = Paragraph::new(app.config.get_logo())
                .block(Block::default())
                .style(
                    app.config
                        .get_style()
                        .add_modifier(tui::style::Modifier::BOLD),
                )
                .alignment(Alignment::Center);
            frame.render_widget(logo, small_rect(frame.size()));
        }
        let area = small_rect(frame.size());
        let opts = app.opts.clone();
        let display_opts = handle_display_options(&opts);
        frame.render_widget(
            Paragraph::new(display_opts)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Double)
                        .title_style(Style::new().bold().italic())
                        .title("-Request Options-"),
                )
                .fg(app.config.get_fg_color())
                .bg(app.config.get_bg_color())
                .style(Style::default())
                .alignment(Alignment::Left),
            area,
        );
        // ******************************************************************************************************
    } else {
        let area = small_rect(frame.size());
        let response = app.response.clone().unwrap();
        let paragraph = Paragraph::new(Text::from(response.as_str()))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .border_style(Style::new().bold()),
            )
            .style(app.config.get_style())
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, area);
    }
    // We pass this off where we match on the current screen and render what we need to
    handle_screen(app, frame, app.current_screen.clone());
}

pub fn handle_screen_defaults<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let mut items: Option<Vec<String>> = None;
    match app.current_screen {
        Screen::SavedKeys => {
            items = Some(
                app.get_saved_keys()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            );
        }
        Screen::SavedCommands => {
            items = Some(
                app.get_saved_commands()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|x| format!("{:?}", x))
                    .collect::<Vec<String>>(),
            );
        }
        _ => {}
    }
    let menu_options = app.current_screen.get_list(items);
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
        Screen::Downloads(_) => (&DEFAULT_MENU_PARAGRAPH, &DOWNLOAD_MENU_TITLE),
        Screen::SavedKeys => (&API_KEY_PARAGRAPH, &API_KEY_TITLE),
        Screen::HeaderAddRemove => (&DEFAULT_MENU_PARAGRAPH, &DEFAULT_MENU_TITLE),
        _ => (&DEFAULT_MENU_PARAGRAPH, &DEFAULT_MENU_TITLE),
    };
    frame.render_widget(
        render_header_paragraph(paragraph, title, app.config.get_style()),
        frame.size(),
    );
}

pub fn handle_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, screen: Screen) {
    match screen {
        Screen::Home => handle_home_screen(app, frame),
        Screen::Method => handle_method_select_screen(app, frame),
        Screen::ViewBody => {
            let area = default_rect(frame.size());
            let response = app.response.clone().unwrap_or_default();
            let paragraph = Paragraph::new(Text::from(response.as_str()))
                .style(app.config.get_style())
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, area);
        }
        Screen::Downloads(e) => {
            if is_prompt(&e) {
                handle_downloads_screen(app, frame, &e);
            } else {
                handle_downloads_screen(app, frame, "");
            }
        }
        //
        // REQUEST MENU *********************************************************
        Screen::RequestMenu(e) => {
            if is_prompt(&e) {
                handle_request_menu_screen(app, frame, e);
            } else {
                handle_request_menu_screen(app, frame, "".to_string());
            }
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
            app.set_response(&resp);
            handle_response_screen(app, frame, resp.to_string());
        }
        Screen::SavedCommands => {
            handle_saved_commands_screen(app, frame);
        }
        Screen::Error(e) => {
            handle_error_screen(app, frame, e);
        }
        Screen::MoreFlags => {
            handle_more_flags_screen(app, frame);
        }
        Screen::SavedKeys => {
            handle_saved_keys_screen(app, frame);
        }
        Screen::CmdMenu(cmd) => {
            handle_alert_menu(app, frame, cmd);
        }
        Screen::RequestBodyInput => handle_req_body_input_screen(app, frame, InputOpt::RequestBody),
        Screen::KeysMenu(cmd) => handle_key_menu(app, frame, cmd),
        _ => {}
    }
}

fn is_prompt(e: &str) -> bool {
    e.to_lowercase().contains("error") || e.to_lowercase().contains("alert")
}

fn handle_display_options(opts: &[AppOptions]) -> Vec<Line> {
    opts.iter()
        .map(|x| Line::from(x.get_value()))
        .collect::<Vec<Line>>()
}

pub fn render_header_paragraph(
    para: &'static str,
    title: &'static str,
    style: Style,
) -> Paragraph<'static> {
    Paragraph::new(para)
        .block(
            Block::default()
                .title(title)
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Double),
        )
        .style(style)
        .alignment(Alignment::Center)
}
