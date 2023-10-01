use tui::backend::Backend;
use tui::Frame;

use crate::app::App;
use crate::ui::widgets::boxes::default_rect;
use crate::ui::widgets::menu::menu_widget;

pub fn handle_success_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let area = default_rect(frame.size());
    app.items = app.current_screen.get_opts();
    frame.set_cursor(0, app.cursor as u16);
    frame.render_widget(menu_widget(), area);
}
