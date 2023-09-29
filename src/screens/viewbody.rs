use tui::backend::Backend;
use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::Paragraph;
use tui::Frame;

use crate::app::App;
use crate::ui::widgets::small_rect;

pub fn handle_view_body_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // screen with only the body of the response
    app.items.clear();
    let area = small_rect(frame.size());
    let response = app.response.clone().unwrap();
    let paragraph = Paragraph::new(Text::from(response.as_str()))
        .style(Style::default().fg(Color::Yellow).bg(Color::Black))
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}
