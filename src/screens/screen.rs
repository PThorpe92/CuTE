/*
    Screen Enum And Implementation
*/

use std::fmt::{Display, Formatter};

use crate::display::inputopt::InputOpt;
use crate::display::menuopts::{
    AUTHENTICATION_MENU_OPTIONS, DOWNLOAD_MENU_OPTIONS, MAIN_MENU_OPTIONS, METHOD_MENU_OPTIONS,
    OPTION_PADDING_MAX, OPTION_PADDING_MID, OPTION_PADDING_MIN, REQUEST_MENU_OPTIONS,
    RESPONSE_MENU_OPTIONS,
};
use crossterm::terminal::window_size;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem};

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
    SavedKeys,
    SavedCommands,
    Error(String),
    ViewBody,
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
            Screen::SavedKeys => "Saved Keys",
            Screen::SavedCommands => "My Saved Commands",
            Screen::Error(_) => "Error",
            Screen::ViewBody => "ViewBody",
        };
        write!(f, "{}", screen)
    }
}

pub fn determine_line_size() -> &'static str {
    match window_size() {
        Ok(size) => {
            // if we have the size, we can make the options look better
            if size.width >= 1020 && size.height >= 680 {
                return OPTION_PADDING_MAX;
            } else if size.width >= 800 && size.height >= 600 {
                return OPTION_PADDING_MID;
            }
            return OPTION_PADDING_MIN;
        }
        Err(_) => return OPTION_PADDING_MID,
    }
}

impl<'a> Screen {
    pub fn default() -> Self {
        Screen::Home
    }

    pub fn get_opts(&self, items: Option<Vec<String>>) -> Vec<ListItem<'a>> {
        match &self {
            Screen::Home => {
                return MAIN_MENU_OPTIONS
                    .iter()
                    .map(|x| String::from(format!("{}{}", x, determine_line_size())))
                    .map(|i| ListItem::new(i.clone()))
                    .collect();
            }
            Screen::Method => {
                return METHOD_MENU_OPTIONS
                    .iter()
                    .map(|x| String::from(format!("{}{}", x, determine_line_size())))
                    .map(|i| ListItem::new(i.clone()))
                    .collect();
            }
            Screen::HeaderAddRemove => {
                return METHOD_MENU_OPTIONS
                    .iter()
                    .map(|x| String::from(format!("{}{}", x, determine_line_size())))
                    .map(|i| ListItem::new(i.clone()))
                    .collect();
            }
            Screen::RequestMenu(_) => {
                return REQUEST_MENU_OPTIONS
                    .iter()
                    .map(|x| String::from(format!("{}{}", x, OPTION_PADDING_MID)))
                    .map(|i| ListItem::new(i.clone()))
                    .collect();
            }
            Screen::SavedCommands => {
                return items
                    .unwrap_or(vec!["No Saved Commands".to_string()])
                    .iter()
                    .map(|c| ListItem::new(format!("{}{}", c, determine_line_size())))
                    .collect();
            }
            Screen::Response(_) => {
                return RESPONSE_MENU_OPTIONS
                    .iter()
                    .map(|x| String::from(format!("{}{}", x, OPTION_PADDING_MIN)))
                    .map(|i| ListItem::new(i.clone()))
                    .collect();
            }
            Screen::InputMenu(_) => {
                vec![ListItem::new("Input Menu").style(Style::default().fg(Color::Green))]
            }
            Screen::Authentication => {
                return AUTHENTICATION_MENU_OPTIONS
                    .iter()
                    .map(|x| String::from(format!("{}{}", x, determine_line_size())))
                    .map(|i| ListItem::new(i.clone()))
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
                    .map(|x| String::from(format!("{}{}", x, determine_line_size())))
                    .map(|i| ListItem::new(i.clone()))
                    .collect();
            }
            Screen::SavedKeys => {
                return items
                    .unwrap_or(vec!["No Saved Commands".to_string()])
                    .iter()
                    .map(|c| ListItem::new(format!("{}{}", c, determine_line_size())))
                    .collect();
            }
        }
    }

    pub fn get_list(&self, items: Option<Vec<String>>) -> List {
        List::new(self.get_opts(items))
            .block(
                Block::default()
                    .title(self.to_string().clone())
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol("󱋰 ")
    }
}
