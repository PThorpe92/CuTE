/*
    Screen Enum And Implementation
*/

use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem};

use crate::display::inputopt::InputOpt;
use crate::display::menuopts::{
    API_KEY_MENU_OPTIONS, DEBUG_MENU_OPTIONS, INPUT_MENU_OPTIONS, MAIN_MENU_OPTIONS,
    METHOD_MENU_OPTIONS, REQUEST_MENU_OPTIONS, RESPONSE_MENU_OPTIONS, SAVED_COMMAND_OPTIONS,
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
    Success,
    Keys,
    Debug,
    // Debug Screen Which Allows Me To Test 1 Kind Of Screen At A Time
    TestInput(String),
    // Testing Input And Layout
    Commands,
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
                vec![ListItem::new("Downloads").style(Style::default().fg(Color::Green))]
            }
            Screen::Debug => {
                return DEBUG_MENU_OPTIONS
                    .iter()
                    .map(|i| ListItem::new(*i).style(Style::default().fg(Color::LightMagenta)))
                    .collect();
            }
            Screen::TestInput(_) => {
                return INPUT_MENU_OPTIONS
                    .iter()
                    .map(|x| ListItem::new(*x).style(Style::default().fg(Color::LightMagenta)))
                    .collect();
            }
        }
    }

    // Get List Calls Get Opts* Personal Reminder For Lorenzo.
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
            Screen::Method => "Choose an HTTP Method",
            Screen::HeaderAddRemove => "Add or Remove Headers",
            Screen::Keys => "My Saved API Keys",
            Screen::RequestMenu(_) => "Command Request Options",
            Screen::Commands => "My Saved Commands",
            Screen::InputMenu(_) => "User input",
            Screen::Response(_) => "Response",
            Screen::Success => "Success",
            Screen::Error(_) => "Error",
            Screen::ViewBody => "View response body",
            Screen::Downloads => "Downloads",
            Screen::Debug => "Debug Menu",
            Screen::TestInput(_) => "Test Input",
        }
        .to_string()
    }
}
