use crate::curl::Curl;
use crate::wget::Wget;
use crate::Request;
use lazy_static::lazy_static;
use std::error;
use tui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use tui_input::Input;
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone, PartialEq)]

pub enum InputMode {
    Normal,
    Editing,
}

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    pub cursor: usize,
    pub current_screen: Screen,
    pub screen_stack: Vec<Screen>,
    pub command: Option<Command<'a>>,
    pub selected: Option<usize>,
    pub input: Input,
    pub messages: Vec<String>,
    pub input_mode: InputMode,
    pub items: Vec<ListItem<'a>>,
    pub state: Option<ListState>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Screen {
    Home,
    Command(String),
    CurlMenu(String),
    WgetMenu(String),
    CustomMenu(String),
    InputMenu(usize),
    Keys,
    Saved,
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
            Screen::Command(_) => {
                return COMMAND_MENU_OPTIONS
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
            Screen::CurlMenu(_) => {
                return CURL_MENU_OPTIONS
                    .iter()
                    .map(|i| ListItem::new(*i))
                    .collect();
            }
            Screen::WgetMenu(_) => {
                return WGET_MENU_OPTIONS
                    .iter()
                    .map(|i| ListItem::new(*i))
                    .collect();
            }
            Screen::CustomMenu(_) => {
                return HTTP_MENU_OPTIONS
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
            Screen::InputMenu(ind) => {
                return INPUT_MENU_OPTIONS
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
                        if i == *ind {
                            return ListItem::new(*x).style(Style::default().fg(Color::Yellow));
                        }
                        return ListItem::new(&**x);
                    })
                    .collect()
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
            Screen::Command(_) => "Command",
            Screen::Keys => "My Saved API Keys",
            Screen::CurlMenu(_) => "Curl",
            Screen::WgetMenu(_) => "Wget",
            Screen::CustomMenu(_) => "Custom HTTP Request",
            Screen::Saved => "My Saved Commands",
            Screen::InputMenu(_) => "User input",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command<'a> {
    Curl(Curl<'a>),
    Wget(Wget),
    Custom(Request),
}
impl<'a> Command<'a> {
    pub fn default(curl: Curl<'a>) -> Self {
        Command::Curl(curl)
    }
    pub fn set_method(&mut self, method: String) {
        match self {
            Command::Curl(curl) => {
                curl.set_method(method);
            }
            Command::Wget(wget) => {
                wget.set_method(method.to_string());
            }
            Command::Custom(req) => {
                req.method = method;
            }
        }
    }

    pub fn set_url(&mut self, url: String) {
        match self {
            Command::Curl(curl) => {
                curl.set_url(url.clone());
                return;
            }
            Command::Wget(wget) => {
                wget.set_url(url);
                return;
            }
            Command::Custom(req) => {
                req.url = url;
                return;
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Command::Curl(_) => "Curl",
            Command::Wget(_) => "Wget",
            Command::Custom(_) => "Custom",
        }
        .to_string()
    }
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self {
            running: true,
            cursor: 0,
            screen_stack: vec![Screen::Home],
            selected: None,
            command: None,
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            items: Vec::from(Screen::Home.get_opts()),
            input: Input::default(),
            state: None,
            current_screen: Screen::Home,
        }
    }
}

impl<'a> App<'_> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn goto_screen(&mut self, screen: Screen) {
        self.screen_stack.push(screen.clone());
        self.current_screen = screen.clone();
        self.cursor = 0;
        self.items = screen.get_opts();
        self.selected = None;
        return;
    }

    pub fn go_back_screen(&mut self) {
        if self.screen_stack.len() == 1 {
            return;
        } else {
            self.cursor = 0;
            self.selected = None;
            self.current_screen = self.screen_stack.last().unwrap().clone();
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn move_cursor_down(&mut self) {
        if self.items.len() == 0 || self.cursor >= self.items.len() {
            return;
        }
        if let Some(res) = self.cursor.checked_add(1) {
            self.cursor = res;
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.items.len() == 0 {
            return;
        }
        if let Some(res) = self.cursor.checked_sub(1) {
            self.cursor = res;
        }
    }

    pub fn select_item(&mut self) {
        let state = self.state.as_mut().unwrap();
        if let Some(selected) = state.selected() {
            // ^^^ returns usize index
            self.selected = Some(selected);
        }
    }
}

lazy_static! {
pub static ref CURL_MENU_OPTIONS: Vec<&'static str> = vec![
    "Add a URL\n \n",
    "Add Authentication\n \n",
    "Add Headers\n \n",
    "Enable verbose output\n \n",
    "Specify request output file\n \n",
    "Add Request Body\n \n",
    "Execute command\n \n",
];
pub static ref WGET_MENU_OPTIONS: Vec<&'static str> = vec![
    "Add a URL\n \n",
    "Add Authentication\n \n ",
    "Add Headers\n \n",
    "Enable verbose output\n \n",
    "Specify download output file\n \n",
    "Specify recursive download\n \n",
    "Add Request Body\n \n",
    "Execute command\n \n",
];

pub static ref COMMAND_MENU_OPTIONS: Vec<&'static str> = vec![
    "Choose an HTTP method:\n \n",
    "GET\n \n",
    "POST\n \n",
    "PUT\n \n",
    "DELETE\n \n",
    "PATCH\n \n",
];

pub static ref HTTP_MENU_OPTIONS: Vec<&'static str> = vec![
    "Add a URL\n \n",
    "Authentication\n \n",
    "Add Headers\n \n",
    "Specify response output file\n \n ",
    "Add Request Body\n \n",
    "Send Request \n \n",
];

pub static ref AUTHENTICATION_MENU_OPTIONS: Vec<&'static str> = vec![
    "Basic\n \n",
    "Bearer\n \n",
    "Digest\n \n",
    "Hawk\n \n",
    "OAuth\n \n",
    "AWS Signature\n \n",
    "NTLM\n \n",
    "Kerberos\n \n",
    "SPNEGO\n \n",
    "Custom\n \n",
];

pub static ref INPUT_MENU_OPTIONS: Vec<&'static str> = vec![
        "Please enter a URL for your request",
        "Please specify your request headers",
        "Please enter your request body",
];

pub static ref MAIN_MENU_OPTIONS: Vec<&'static str> = vec![
    "Build and run a new cURL command\n  \n",
    "Build and run a new wget command\n  \n",
    "Build/send new custom HTTP request\n  \n",
    "View my stored API keys\n  \n",
    "View or execute my saved commands\n  \n",
];

// TODO: keys and saved commands menus
// Filler for now until these are implemented:
// obviously we need to be fetching these from the db
pub static ref API_KEY_MENU_OPTIONS: Vec<&'static str> = vec![
    "Add a new key\n \n",
    "View my keys\n \n",
    "Delete a key\n \n",
];
pub static ref SAVED_COMMAND_OPTIONS: Vec<&'static str> = vec![
    "Add a new saved command\n \n",
    "View my saved commands\n \n",
    "Delete a saved command\n \n",
];
}
