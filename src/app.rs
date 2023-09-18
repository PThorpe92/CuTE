use crate::curl::{Curl, CurlFlag, CurlFlagType};
use crate::wget::Wget;
use crate::Request;
use lazy_static::lazy_static;
use std::error;
use tokio::runtime;
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
    pub opts: Vec<DisplayOpts>,
    pub input: Input,
    pub messages: Vec<String>,
    pub input_mode: InputMode,
    pub items: Vec<ListItem<'a>>,
    pub state: Option<ListState>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Screen {
    Home,
    Method(String),
    RequestMenu(String),
    InputMenu(usize),
    Response(String),
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
            Screen::Method(_) => {
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
                    .collect()
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
                    .collect()
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
            Screen::Method(_) => "Choose an HTTP Method",
            Screen::Keys => "My Saved API Keys",
            Screen::RequestMenu(_) => "Command Request Options",
            Screen::Saved => "My Saved Commands",
            Screen::InputMenu(_) => "User input",
            Screen::Response(_) => "Response",
        }
        .to_string()
    }
}

/// Here are the options that require us to display a box letting
/// the user know that they have selected that option.
#[derive(Debug, Clone, PartialEq)]
pub enum DisplayOpts {
    Verbose,
    URL(String),
    Outfile(String),
    SaveCommand,
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

    pub fn set_outfile(&mut self, file: String) {
        match self {
            Command::Curl(curl) => {
                curl.add_flag(CurlFlag::Output(""), Some(file));
                return;
            }
            Command::Wget(wget) => {
                wget.set_output(file);
                return;
            }
            Command::Custom(req) => {
                req.output = Some(file.clone());
                return;
            }
        }
    }

    pub fn add_headers(&mut self, headers: (String, String)) {
        match self {
            Command::Custom(req) => {
                req.add_headers(headers);
                return;
            }
            _ => {
                return;
            }
        }
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        match self {
            Command::Curl(curl) => {
                if verbose {
                    curl.add_flag(CurlFlag::Verbose("-v"), None);
                } else {
                    curl.remove_flag(CurlFlag::Verbose(""));
                }
            }
            Command::Wget(wget) => {
                wget.set_verbose(verbose);
                return;
            }
            Command::Custom(_) => {
                return;
            }
        }
    }

    pub async fn execute(&mut self) -> Result<String, std::io::Error> {
        match self {
            Command::Curl(curl) => Ok(curl.execute().unwrap_or("".to_string())),
            Command::Wget(wget) => Ok(wget.execute().unwrap_or("".to_string())),
            Command::Custom(req) => Ok(req.send_request().await.unwrap()),
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
            opts: Vec::new(),
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

    pub fn has_display_option(&self, opt: DisplayOpts) -> bool {
        match self.opts.iter().find(|x| x == &&opt) {
            Some(_) => true,
            None => false,
        }
    }

    // user selects once, we add. twice we remove.
    pub fn add_display_option(&mut self, opt: DisplayOpts) {
        if !self.has_display_option(opt.clone()) {
            self.opts.push(opt);
        } else {
            self.opts.retain(|x| x != &opt);
        }
    }
}

lazy_static! {
pub static ref REQUEST_MENU_OPTIONS: Vec<&'static str> = vec![
    "Add a URL\n \n",
    "Add Authentication\n \n",
    "Add Headers\n \n",
    "Enable verbose output\n \n",
    "Specify request output file\n \n",
    "Add Request Body\n \n",
    "Execute command\n \n",
    "Save this command\n \n",
    "Recursive download (wget only)\n \n"
];

pub static ref METHOD_MENU_OPTIONS: Vec<&'static str> = vec![
    "Choose an HTTP method:\n \n",
    "GET\n \n",
    "POST\n \n",
    "PUT\n \n",
    "DELETE\n \n",
    "PATCH\n \n",
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
pub static ref RESPONSE_MENU_OPTIONS: Vec<&'static str> = vec![
        "Write to file?\n \n",
        "View response headers\n \n",
        "View response body\n \n",
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
