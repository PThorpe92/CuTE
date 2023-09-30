use tui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::display::displayopts::DisplayOpts;
use crate::screens::debug::handle_debug_screen;
use crate::screens::downloads::handle_downloads_screen;
use crate::screens::home::handle_home_screen;
use crate::screens::input::url::handle_url_input_screen;
use crate::screens::keys::handle_api_key_screen;
use crate::screens::method::handle_method_select_screen;
use crate::screens::request::handle_request_menu_screen;
use crate::screens::response::handle_response_screen;

use crate::display::inputopt::InputOpt;
use crate::display::menuopts::METHOD_MENU_OPTIONS;
use crate::request::command::Command;
use crate::request::curl::Curl;
use crate::request::wget::Wget;

use crate::screens::screen::Screen;
use crate::screens::success::handle_success_screen;
use crate::screens::viewbody::handle_view_body_screen;
use crate::ui::widgets::boxes::small_rect;

pub static CURL: &str = "curl";
pub static WGET: &str = "wget";
pub static CUSTOM: &str = "custom";

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    if app.response.is_none() {
        // Render Display Options *******************************************
        let mut display_opts = String::new();
        app.opts.iter().for_each(|opt| match opt {
            DisplayOpts::Verbose => {
                display_opts.push_str("- Verbose\n");
            }
            DisplayOpts::URL(url) => {
                let url_str = format!("- URL: {}\n", &url);
                display_opts.push_str(url_str.as_str());
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
            .style(Style::default().fg(Color::Cyan).bg(Color::Yellow))
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

    // Render the current screen
    match &app.current_screen.clone() {
        // HOME SCREEN ******************************************************
        Screen::Home => {
            handle_home_screen(app, frame);
        }

        // METHOD SCREEN ****************************************************
        Screen::Method => {

            handle_method_select_screen(app, frame);
        }

        // DOWNLOAD SCREEN **************************************************
        Screen::Downloads => {
            handle_downloads_screen(app, frame);
        }

        // KEYS SCREEN **********************************************
        Screen::Keys => {
            handle_api_key_screen(app, frame);
        }

        // REQUEST MENU SCREEN **********************************************
        Screen::RequestMenu(_) => {
            handle_request_menu_screen(app, frame);
        }

        // SUCESSS SCREEN *********************************************************
        Screen::Success => {
            handle_success_screen(app, frame);
        }

        // INPUT MENU SCREEN *****************************************************
        Screen::InputMenu(opt) => {
            //render_input_screen(app, frame, opt.clone());
        }

        // RESPONSE SCREEN ******************************************************
        Screen::Response(resp) => {
            handle_response_screen(app, frame, resp.clone());
        }

        // VIEW BODY ********************************************************************
        Screen::ViewBody => {
            handle_view_body_screen(app, frame);
        }

        // DEBUG SCREEN ********************************************************************
        Screen::Debug => {
            handle_debug_screen(app, frame);
        }

        // TEST INPUT ********************************************************************
        Screen::TestInput(_) => {
            handle_url_input_screen(app, frame);
        }

        _ => {}
    }
}


/* Never Used
fn success_paragraph() -> Paragraph<'static> {
    Paragraph::new("Command successfully saved\n")
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
 */
