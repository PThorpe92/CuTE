use tui::backend::Backend;
use tui::Frame;

use crate::app::App;
use crate::ui::render::{render_header_paragraph, HOME_MENU_PARAGRAPH};
use crate::ui::widgets::boxes::default_rect;

const SUCCESS_TITLE: &'static str = "* CuTE *\n* Success! *";
pub fn handle_success_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let area = default_rect(frame.size());
    app.items = app.current_screen.get_opts();
    frame.set_cursor(0, app.cursor as u16);
    frame.render_widget(
        render_header_paragraph(HOME_MENU_PARAGRAPH, SUCCESS_TITLE),
        area,
    );
}
