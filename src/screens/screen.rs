/*
    Screen Enum And Implementation
*/

use std::fmt::{Display, Formatter};

use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem};

use crate::display::inputopt::InputOpt;
use crate::display::menuopts::{
    API_KEY_MENU_OPTIONS, AUTHENTICATION_MENU_OPTIONS, DEBUG_MENU_OPTIONS, DOWNLOAD_MENU_OPTIONS,
    INPUT_MENU_OPTIONS, MAIN_MENU_OPTIONS, METHOD_MENU_OPTIONS, REQUEST_MENU_OPTIONS,
    RESPONSE_MENU_OPTIONS, SAVED_COMMAND_OPTIONS,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Screen {
    Home,
    Method,
    Downloads,
    HeaderAddRemove,
    RequestMenu(String),
    InputMenu(InputOpt),
    Response(String),
    Authentication,
    Success,
    Keys,
    Commands,
    Error(String),
    ViewBody,
    Debug,
    URLInput,
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let screen = match self {
            Screen::Home => "Home",
            Screen::Method => "Method",
            Screen::Downloads => "Downloads",
            Screen::HeaderAddRemove => "HeaderAddRemove",
            Screen::RequestMenu(_) => "RequestMenu",
            Screen::InputMenu(_) => "InputMenu",
            Screen::Response(_) => "Response",
            Screen::Authentication => "Authentication",
            Screen::Success => "Success",
            Screen::Keys => "Keys",
            Screen::Commands => "Commands",
            Screen::Error(_) => "Error",
            Screen::ViewBody => "ViewBody",
            Screen::Debug => "Debug",
            Screen::URLInput => "URLInput",
        };
        write!(f, "{}", screen)
    }
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
            Screen::Method => {
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
            Screen::Commands => {
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
            Screen::Authentication => {
                return AUTHENTICATION_MENU_OPTIONS
                    .iter()
                    .map(|i| ListItem::new(*i))
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
            Screen::Downloads => {
                return DOWNLOAD_MENU_OPTIONS
                    .iter()
                    .map(|i| ListItem::new(*i))
                    .collect();
            }
            Screen::Debug => {
                // Menu For Debug Screens
                return DEBUG_MENU_OPTIONS
                    .iter()
                    .map(|i| ListItem::new(*i))
                    .collect();
            }
            Screen::URLInput => {
                vec![ListItem::new("URL Input").style(Style::default().fg(Color::Green))]
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
}
