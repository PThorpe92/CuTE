use tui::backend::Backend;
use tui::widgets::ListState;
use tui::Frame;

use crate::app::App;
use crate::display::inputopt::InputOpt;
use crate::request::curl::AuthKind;
use crate::screens::screen::Screen;
use crate::ui::render::{render_header_paragraph, HOME_MENU_PARAGRAPH};
use crate::ui::widgets::boxes::default_rect;

const AUTH_MENU_TITLE: &'static str = "Authentication Menu";
pub fn handle_authentication_screen<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let area = default_rect(frame.size());
    let new_list = app.current_screen.get_list();
    let mut state = ListState::with_selected(ListState::default(), Some(app.cursor));
    app.items = app.current_screen.get_opts();
    app.state = Some(state.clone());
    app.state.as_mut().unwrap().select(Some(app.cursor));
    frame.set_cursor(0, app.cursor as u16);
    frame.render_stateful_widget(new_list, area, &mut state);
    frame.render_widget(
        render_header_paragraph(HOME_MENU_PARAGRAPH, AUTH_MENU_TITLE),
        frame.size(),
    );
    if let Some(num) = app.selected {
        match num {
            0 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthKind::Basic(
                String::new(),
            )))),
            1 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthKind::Bearer(
                String::new(),
            )))),
            2 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthKind::Digest(
                String::new(),
            )))),
            3 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthKind::AwsSigv4(
                String::new(),
            )))),
            4 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthKind::Spnego(
                String::new(),
            )))),
            5 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthKind::Kerberos(
                String::new(),
            )))),
            6 => app.goto_screen(Screen::InputMenu(InputOpt::Auth(AuthKind::Ntlm(
                String::new(),
            )))),
            _ => {}
        }
    }
}
