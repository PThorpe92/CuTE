use tui::backend::Backend;
use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::{ListState, Paragraph};
use tui::Frame;

use crate::app::App;
use crate::screens::screen::Screen;
use crate::ui::widgets::boxes::{default_rect, small_alert_box};

pub fn handle_response_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, resp: String) {
    let area = default_rect(small_alert_box(frame.size()));
    let new_list = app.current_screen.get_list();
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    let paragraph = Paragraph::new(Text::from(resp.as_str()))
        .style(Style::default().fg(Color::Yellow).bg(Color::Black))
        .alignment(Alignment::Center);
    if !app.items.is_empty() {
        app.items.clear();
    }
    app.items = app.current_screen.get_opts();
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(new_list, area, &mut state);
    let area_2 = small_alert_box(frame.size());
    frame.render_widget(paragraph, area_2);
    match app.selected {
        Some(num) => match num {
            0 => {
                app.goto_screen(Screen::Home); // TODO: FIX INPUT
            }
            1 => {
                app.goto_screen(Screen::Commands);
            }
            2 => {
                app.goto_screen(Screen::ViewBody);
            }
            _ => {}
        },
        None => {}
    }
}
