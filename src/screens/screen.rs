use crate::display::inputopt::InputOpt;
use crate::display::menuopts::{
    AUTHENTICATION_MENU_OPTIONS, CMD_MENU_OPTIONS, COLLECTION_ALERT_MENU_OPTS,
    COLLECTION_MENU_OPTIONS, COOKIE_MENU_OPTIONS, HEADER_MENU_OPTIONS, KEY_MENU_OPTIONS,
    MAIN_MENU_OPTIONS, METHOD_MENU_OPTIONS, MORE_FLAGS_MENU, NEWLINE, OPTION_PADDING_MAX,
    OPTION_PADDING_MID, OPTION_PADDING_MIN, REQUEST_MENU_OPTIONS, RESPONSE_MENU_OPTIONS,
};
use std::fmt::{Display, Formatter};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem};

#[derive(Debug, Default, PartialEq, Clone)]
pub enum Screen {
    #[default]
    Home,
    Method,
    HeaderAddRemove,
    RequestMenu(Option<InputOpt>),
    InputMenu(InputOpt),
    Response(String),
    SavedCollections(Option<InputOpt>),
    ViewSavedCollections,
    Authentication,
    Success,
    SavedKeys(Option<InputOpt>),
    ColMenu(i32),
    // takes optional collection id
    SavedCommands(Option<i32>),
    Error(String),
    ViewBody,
    MoreFlags,
    Headers,
    CmdMenu(i32),
    KeysMenu(usize),
    RequestBodyInput,
    CookieOptions,
}
impl Screen {
    pub fn is_input_screen(&self) -> bool {
        match self {
            Screen::RequestMenu(opt) => opt.is_some(),
            Screen::InputMenu(_) => true,
            Screen::SavedKeys(opt) => opt.is_some(),
            Screen::RequestBodyInput => true,
            Screen::SavedCollections(opt) => opt.is_some(),
            _ => false,
        }
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let screen = match self {
            Screen::Home => "Home",
            Screen::Method => "Method",
            Screen::HeaderAddRemove => "HeaderAddRemove",
            Screen::RequestMenu(_) => "RequestMenu",
            Screen::InputMenu(_) => "InputMenu",
            Screen::Response(_) => "Response",
            Screen::Authentication => "Authentication",
            Screen::Success => "Success",
            Screen::SavedKeys(_) => "Saved Keys",
            Screen::SavedCommands(_) => "My Saved Commands",
            Screen::Error(_) => "Error",
            Screen::ViewBody => "ViewBody",
            Screen::MoreFlags => "MoreFlags",
            Screen::Headers => "Headers",
            Screen::CmdMenu(_) => "CmdMenu",
            Screen::KeysMenu(_) => "KeysMenu",
            Screen::RequestBodyInput => "RequestBodyInput",
            Screen::SavedCollections(_) => "Saved Collections",
            Screen::ViewSavedCollections => "View Saved Collections",
            Screen::ColMenu(_) => "Collection Menu",
            Screen::CookieOptions => "Cookie Options",
        };
        write!(f, "{}", screen)
    }
}

fn determine_line_size(len: usize) -> &'static str {
    match len {
        len if len <= 4 => OPTION_PADDING_MAX,
        len if len < 8 => OPTION_PADDING_MID,
        _ => OPTION_PADDING_MIN,
    }
}
impl<'a> Screen {
    pub fn get_opts(&self, items: Option<Vec<String>>) -> Vec<ListItem<'a>> {
        match &self {
            Screen::Home => {
                let len = MAIN_MENU_OPTIONS.len();
                MAIN_MENU_OPTIONS
                    .iter()
                    .map(|x| format!("{}{}", x, determine_line_size(len)))
                    .map(ListItem::new)
                    .collect()
            }
            Screen::Method => {
                let len = METHOD_MENU_OPTIONS.len();
                METHOD_MENU_OPTIONS
                    .iter()
                    .map(|x| format!("{}{}", x, determine_line_size(len)))
                    .map(ListItem::new)
                    .collect()
            }
            Screen::HeaderAddRemove => {
                let len = METHOD_MENU_OPTIONS.len();
                METHOD_MENU_OPTIONS
                    .iter()
                    .map(|x| format!("{}{}", x, determine_line_size(len)))
                    .map(ListItem::new)
                    .collect()
            }
            Screen::RequestMenu(_) => {
                let len = REQUEST_MENU_OPTIONS.len();
                REQUEST_MENU_OPTIONS
                    .iter()
                    .map(|x| format!("{}{}", x, determine_line_size(len)))
                    .map(ListItem::new)
                    .collect()
            }
            Screen::SavedCommands(_) => {
                let len = REQUEST_MENU_OPTIONS.len();
                items
                    .unwrap_or(vec!["No Saved Commands".to_string()])
                    .iter()
                    .map(|c| ListItem::new(format!("{}{}", c, determine_line_size(len))))
                    .collect()
            }
            Screen::Response(_) => RESPONSE_MENU_OPTIONS
                .iter()
                .map(|x| format!("{}{}", x, OPTION_PADDING_MID))
                .map(ListItem::new)
                .collect(),
            Screen::InputMenu(_) => {
                vec![ListItem::new("Input Menu").style(Style::default().fg(Color::Green))]
            }
            Screen::Headers => HEADER_MENU_OPTIONS
                .iter()
                .map(|x| format!("{}{}", x, OPTION_PADDING_MID))
                .map(ListItem::new)
                .collect(),
            Screen::Authentication => {
                let len = AUTHENTICATION_MENU_OPTIONS.len();
                AUTHENTICATION_MENU_OPTIONS
                    .iter()
                    .map(|x| format!("{}{}", x, determine_line_size(len)))
                    .map(ListItem::new)
                    .collect()
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
            Screen::RequestBodyInput => {
                vec![ListItem::new("Request Body Input").style(Style::default().fg(Color::Green))]
            }
            Screen::CmdMenu(_) => CMD_MENU_OPTIONS
                .iter()
                .map(|i| ListItem::new(format!("{i}{}", NEWLINE)))
                .collect(),
            Screen::ColMenu(_) => COLLECTION_ALERT_MENU_OPTS
                .iter()
                .map(|i| ListItem::new(*i))
                .collect(),
            Screen::SavedKeys(_) => {
                let mut len = 0;
                if items.is_some() {
                    len = items.as_ref().unwrap().len();
                }
                items
                    .unwrap_or(vec!["No Saved Keys".to_string()])
                    .iter()
                    .map(|c| ListItem::new(format!("{}{}", c, determine_line_size(len))))
                    .collect()
            }
            Screen::KeysMenu(_) => KEY_MENU_OPTIONS
                .iter()
                .map(|i| ListItem::new(format!("{}{}", i, NEWLINE)))
                .collect(),
            Screen::MoreFlags => {
                let len = MORE_FLAGS_MENU.len();
                MORE_FLAGS_MENU
                    .iter()
                    .map(|i| {
                        ListItem::new(format!("{}{}", i, determine_line_size(len)))
                            .style(Style::default().fg(Color::Red))
                    })
                    .collect()
            }
            Screen::ViewSavedCollections => items
                .unwrap_or(vec!["No Collections".to_string()])
                .iter()
                .map(|c| ListItem::new(format!("{}{}", c, OPTION_PADDING_MIN)))
                .collect(),

            Screen::SavedCollections(_) => COLLECTION_MENU_OPTIONS
                .iter()
                .map(|i| ListItem::new(format!("{}{}", i, OPTION_PADDING_MAX)))
                .collect(),

            Screen::CookieOptions => COOKIE_MENU_OPTIONS
                .iter()
                .map(|c| ListItem::from(format!("{}{}", c, OPTION_PADDING_MID)))
                .collect(),
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
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::REVERSED)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::ITALIC),
            )
            .highlight_symbol("󱋰 ")
    }
}
