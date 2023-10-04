use crate::app::App;
use crate::display::displayopts::DisplayOpts;
use crate::screens::handle_screens::handle_screen;
use crate::ui::widgets::boxes::small_rect;

use tui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub const CURL: &'static str = "curl";
pub const WGET: &'static str = "wget";
pub const CUSTOM: &'static str = "custom";
pub const HOME_MENU_PARAGRAPH: &'static str =
    "\nPress q to exit \n Press Enter to select \n Please select a Menu item\n";
pub const HOME_MENU_TITLE: &'static str = "* CuTE *";
pub const DOWNLOAD_MENU_TITLE: &'static str = "* CuTE ** Downloads *";
pub const SUCCESS_MESSAGE: &'static str = "Command saved successfully";

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
                display_opts.push_str("- Command will be saved");
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
    handle_screen(app, frame, app.current_screen.clone());
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
