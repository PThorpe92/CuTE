/*
    Screen Enum And Implementation
*/

use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem};

use crate::display::inputopt::InputOpt;
use crate::display::menuopts::{
    API_KEY_MENU_OPTIONS, INPUT_MENU_OPTIONS, MAIN_MENU_OPTIONS, METHOD_MENU_OPTIONS,
    REQUEST_MENU_OPTIONS, RESPONSE_MENU_OPTIONS, SAVED_COMMAND_OPTIONS,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Screen {
    Home,
    Method(String),
    HeaderAddRemove,
    RequestMenu(String),
    InputMenu(InputOpt),
    Response(String),
    Success,
    Keys,
    Saved,
    Error(String),
    ViewBody,
}

impl<'a> Screen {
    pub fn default() -> Self {
        Screen::Home
    }

    pub fn get_opts(&self) -> Vec<ListItem<'a>> {
        match &self {
            Screen::Home => {
                return MAIN_MENU_OPTIONS
                    .iter()
                    .map(|i| ListItem::new(*i))
                    .collect();
            }
            Screen::Method(_) => {
                return METHOD_MENU_OPTIONS
                    .iter()
                    .map(|i| ListItem::new(*i))
                    .collect();
            }
            Screen::HeaderAddRemove => {
                return METHOD_MENU_OPTIONS
                    .iter()
                    .map(|i| ListItem::new(*i))
                    .collect();
            }
            Screen::Keys => {
                return API_KEY_MENU_OPTIONS
                    .iter()
                    .map(|i| ListItem::new(*i))
                    .collect();
            }
            Screen::RequestMenu(_) => {
                return REQUEST_MENU_OPTIONS
                    .iter()
                    .map(|i| ListItem::new(*i))
                    .collect();
            }
            Screen::Saved => {
                return SAVED_COMMAND_OPTIONS
                    .iter()
                    .map(|i| ListItem::new(*i))
                    .collect();
            }
            Screen::Response(_) => {
                return RESPONSE_MENU_OPTIONS
                    .iter()
                    .map(|i| ListItem::new(*i))
                    .collect();
            }
            Screen::InputMenu(_) => {
                return INPUT_MENU_OPTIONS
                    .iter()
                    .map(|x| ListItem::new(*x))
                    .collect();
            }
            Screen::Success => {
                vec![ListItem::new("Success!").style(Style::default().fg(Color::Green))]
            }
            Screen::Error(_) => {
                vec![ListItem::new("Error!").style(Style::default().fg(Color::Red))]
            }
            Screen::ViewBody => {
                vec![ListItem::new("View Body").style(Style::default().fg(Color::Green))]
            }
        }
    }

    pub fn get_list(&self) -> List {
        List::new(self.get_opts())
            .block(
                Block::default()
                    .title(self.to_string().clone())
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol("->")
    }

    pub fn to_string(&self) -> String {
        match self {
            Screen::Home => "Main Menu",
            Screen::Method(_) => "Choose an HTTP Method",
            Screen::HeaderAddRemove => "Add or Remove Headers",
            Screen::Keys => "My Saved API Keys",
            Screen::RequestMenu(_) => "Command Request Options",
            Screen::Saved => "My Saved Commands",
            Screen::InputMenu(_) => "User input",
            Screen::Response(_) => "Response",
            Screen::Success => "Success",
            Screen::Error(_) => "Error",
            Screen::ViewBody => "View response body",
        }
        .to_string()
    }
}
