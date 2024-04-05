use super::collections::handle_collection_alert_menu;
use super::request::handle_request_menu_screen;
use super::saved_keys::handle_key_menu;
use super::*;
use crate::display::inputopt::InputOpt;
use crate::display::menuopts::{
    API_KEY_PARAGRAPH, API_KEY_TITLE, AUTH_MENU_TITLE, DEFAULT_MENU_PARAGRAPH, DEFAULT_MENU_TITLE,
    ERROR_MENU_TITLE, INPUT_MENU_TITLE, POSTMAN_COLLECTION_TITLE, SAVED_COMMANDS_TITLE,
    SUCCESS_MENU_TITLE, VIEW_BODY_TITLE,
};
use crate::display::AppOptions;
use crate::{app::App, display::menuopts::SAVED_COMMANDS_PARAGRAPH};
use tui::style::Stylize;
use tui::text::Line;
use tui::widgets::ListState;
use tui::widgets::{Block, Borders};
use tui::{
    layout::Alignment,
    style::Style,
    text::Text,
    widgets::{BorderType, Paragraph},
    Frame,
};

use super::{centered_rect, Screen, ScreenArea};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame<'_>) {
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
            frame.render_widget(logo, centered_rect(frame.size(), ScreenArea::Bottom));
        }
        let area = centered_rect(frame.size(), ScreenArea::Bottom);
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
        let area = centered_rect(frame.size(), ScreenArea::Bottom);
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

pub fn handle_screen_defaults(app: &mut App, frame: &mut Frame<'_>) {
    let items = app.get_special_items();
    let menu_options = app.current_screen.get_list(items);
    let area = centered_rect(frame.size(), ScreenArea::Center);

    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.render_stateful_widget(menu_options, area, &mut state);
    let (paragraph, title) = match app.current_screen {
        Screen::Home => (&DEFAULT_MENU_PARAGRAPH, &DEFAULT_MENU_TITLE),
        Screen::SavedCommands(_) => (&SAVED_COMMANDS_PARAGRAPH, &SAVED_COMMANDS_TITLE),
        Screen::Response(_) => (&DEFAULT_MENU_PARAGRAPH, &DEFAULT_MENU_TITLE),
        Screen::InputMenu(_) => (&DEFAULT_MENU_PARAGRAPH, &INPUT_MENU_TITLE),
        Screen::Authentication => (&DEFAULT_MENU_PARAGRAPH, &AUTH_MENU_TITLE),
        Screen::Success => (&DEFAULT_MENU_PARAGRAPH, &SUCCESS_MENU_TITLE),
        Screen::Error(_) => (&DEFAULT_MENU_PARAGRAPH, &ERROR_MENU_TITLE),
        Screen::ViewBody => (&DEFAULT_MENU_PARAGRAPH, &VIEW_BODY_TITLE),
        Screen::SavedKeys => (&API_KEY_PARAGRAPH, &API_KEY_TITLE),
        Screen::HeaderAddRemove => (&DEFAULT_MENU_PARAGRAPH, &DEFAULT_MENU_TITLE),
        Screen::SavedCollections(_) => (&DEFAULT_MENU_PARAGRAPH, &POSTMAN_COLLECTION_TITLE),
        _ => (&DEFAULT_MENU_PARAGRAPH, &DEFAULT_MENU_TITLE),
    };
    frame.render_widget(
        render_header_paragraph(paragraph, title, app.config.get_style()),
        frame.size(),
    );
}

pub fn handle_screen(app: &mut App, frame: &mut Frame<'_>, screen: Screen) {
    match screen {
        // HOME SCREEN *********************************************************
        Screen::Home => {
            handle_screen_defaults(app, frame);
            if let Some(num) = app.selected {
                match num {
                    0 => app.goto_screen(&Screen::Method),
                    1 => app.goto_screen(&Screen::SavedCommands(None)),
                    2 => app.goto_screen(&Screen::SavedCollections(None)),
                    3 => app.goto_screen(&Screen::SavedKeys),
                    _ => {}
                }
            }
        }
        // METHOD MENU SCREEN ***************************************************
        Screen::Method => method::handle_method_select_screen(app, frame),
        // INPUT SCREEN ****************************************************
        Screen::InputMenu(opt) => {
            input::input::handle_default_input_screen(app, frame, opt.clone());
        }
        Screen::ViewBody => {
            let area = centered_rect(frame.size(), ScreenArea::Center);
            let response = app.response.clone().unwrap_or_default();
            let paragraph = Paragraph::new(Text::from(response.as_str()))
                .style(app.config.get_style())
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, area);
        }
        // REQUEST MENU *********************************************************
        Screen::RequestMenu(e) => {
            handle_request_menu_screen(app, frame, e.as_ref());
        }
        // AUTHENTICATION SCREEN ************************************************
        Screen::Authentication => {
            auth::handle_authentication_screen(app, frame);
        }
        // SUCESSS SCREEN *******************************************************
        Screen::Success => handle_screen_defaults(app, frame),
        // RESPONSE SCREEN ******************************************************
        Screen::Response(resp) => {
            app.set_response(&resp);
            response::handle_response_screen(app, frame, resp.to_string());
        }
        Screen::SavedCommands(col) => {
            saved_commands::handle_saved_commands_screen(app, frame, col);
        }
        Screen::Headers => {
            headers::handle_headers_screen(app, frame);
        }
        Screen::ColMenu(selected) => {
            handle_collection_alert_menu(app, frame, selected);
        }
        Screen::Error(e) => {
            error::handle_error_screen(app, frame, e);
        }
        Screen::MoreFlags => {
            more_flags::handle_more_flags_screen(app, frame);
        }
        Screen::SavedKeys => {
            saved_keys::handle_saved_keys_screen(app, frame);
        }
        Screen::CmdMenu(cmd) => {
            saved_commands::handle_alert_menu(app, frame, cmd);
        }
        Screen::CookieOptions => {
            cookies::handle_cookies_menu(app, frame);
        }
        Screen::RequestBodyInput => input::request_body_input::handle_req_body_input_screen(
            app,
            frame,
            InputOpt::RequestBody,
        ),
        Screen::KeysMenu(cmd) => handle_key_menu(app, frame, cmd),
        Screen::SavedCollections(opt) => {
            super::collections::handle_collection_menu(app, frame, opt);
        }
        Screen::ViewSavedCollections => {
            super::collections::handle_collections_screen(app, frame);
        }
        _ => {}
    }
}

fn handle_display_options(opts: &[AppOptions]) -> Vec<Line> {
    opts.iter()
        .map(|x| Line::from(x.get_value()))
        .collect::<Vec<Line>>()
}

#[rustfmt::skip]
pub fn render_header_paragraph(para: &'static str, title: &'static str, style: Style) -> Paragraph<'static> {
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
