use crate::curl::{Curl, CurlFlag};
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
    InputMenu(InputOpt),
    Response(String),
    Success,
    Keys,
    Saved,
    Error(String),
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
            Screen::InputMenu(_) => {
                return INPUT_MENU_OPTIONS
                    .iter()
                    .map(|x| ListItem::new(*x))
                    .collect()
            }
            Screen::Success => {
                vec![ListItem::new("Success!").style(Style::default().fg(Color::Green))]
            }
            Screen::Error(_) => {
                vec![ListItem::new("Error!").style(Style::default().fg(Color::Red))]
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
            Screen::Success => "Success",
            Screen::Error(_) => "Error",
        }
        .to_string()
    }
}

/// Here are the options that require us to display a box letting
/// the user know that they have selected that option.
#[derive(Debug, Clone, PartialEq)]
pub enum DisplayOpts {
    Verbose,
    // TODO: support more headers
    Headers((String, String)),
    URL(String),
    Outfile(String),
    SaveCommand,
    Response(String),
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
            }
            Command::Wget(wget) => {
                wget.set_url(url);
            }
            Command::Custom(req) => {
                req.url = url;
            }
        }
    }

    pub fn set_outfile(&mut self, file: String) {
        match self {
            Command::Curl(curl) => {
                curl.add_flag(CurlFlag::Output(""), Some(file));
            }
            Command::Wget(wget) => {
                wget.set_output(file);
            }
            Command::Custom(req) => {
                req.output = Some(file.clone());
            }
        }
    }

    pub fn add_headers(&mut self, headers: (String, String)) {
        match self {
            Command::Custom(req) => {
                req.add_headers(headers);
            }
            _ => {
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
            }
            Command::Custom(_) => {
            }
        }
    }
    pub fn set_rec_download(&mut self, level: usize) {
        match self {
            Command::Wget(wget) => {
                wget.set_recursive_download(level as u8);
            }
            _ => {
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
    pub fn write_output(&mut self) -> Result<(), std::io::Error> {
        match self {
            Command::Curl(curl) => {
                curl.write_output()?;
                Ok(())
            }
            Command::Wget(wget) => {
                wget.write_output()?;
                Ok(())
            }
            Command::Custom(_) => {
                Ok(())
            }
        }
    }

    pub fn set_response(&mut self, response: String) {
        match self {
            Command::Curl(curl) => {
                curl.set_response(response);
            }
            Command::Wget(wget) => {
                wget.set_response(response);
            }
            Command::Custom(req) => {
                req.set_response(response);
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
            opts: Vec::new(),
            items: Screen::Home.get_opts(),
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
    }

    pub fn go_back_screen(&mut self) {
        match self.screen_stack.pop() {
            // we are not returning to an input menu, so we pop the last element that wasn't an input menu
            Some(Screen::InputMenu(_)) => {
                // we can unwrap, because if we have hit an input menu, it's guaranteed
                self.current_screen = self.screen_stack.last().unwrap().clone();
            }
            Some(_) => {
                self.cursor = 0;
                self.selected = None;
                self.current_screen = self.screen_stack.last().unwrap().clone();
            }
            None => {
            }
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn move_cursor_down(&mut self) {
        if self.items.is_empty() || self.cursor >= self.items.len() {
            return;
        }
        if let Some(res) = self.cursor.checked_add(1) {
            self.cursor = res;
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.items.is_empty() {
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

    // Display option is some state that requires us to display the users
    // current selection on the screen so they know what they have selected
    pub fn has_display_option(&self, opt: DisplayOpts) -> bool {
        self.opts.iter().any(|x| &x == &&opt)
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
#[derive(Debug, Clone, PartialEq)]
pub enum InputOpt {
    URL,
    Headers,
    Output,
    RequestBody,
    RecursiveDownload,
    Authentication,
    Execute,
}
impl InputOpt {
    pub fn to_string(&self) -> String {
        match self {
            InputOpt::URL => "URL",
            InputOpt::Headers => "Headers",
            InputOpt::Output => "Output",
            InputOpt::RequestBody => "Request Body",
            InputOpt::RecursiveDownload => "Recursive Download",
            InputOpt::Authentication => "Authentication",
            InputOpt::Execute => "Execute",
        }
        .to_string()
    }
}

pub enum Auth {
    Basic,
    Bearer,
    Digest,
    Hawk,
    OAuth,
    AWSSignature,
    NTLM,
    Kerberos,
    SPNEGO,
    Custom,
}

lazy_static! {
    pub static ref REQUEST_MENU_OPTIONS: [&'static str; 9] = [
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
    pub static ref METHOD_MENU_OPTIONS: [&'static str; 6] = [
        "Choose an HTTP method:\n \n",
        "GET\n \n",
        "POST\n \n",
        "PUT\n \n",
        "DELETE\n \n",
        "PATCH\n \n",
    ];
    pub static ref AUTHENTICATION_MENU_OPTIONS: [&'static str; 10] = [
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
    pub static ref INPUT_MENU_OPTIONS: [&'static str; 3] = [
        "Please enter a URL for your request",
        "Please specify your request headers",
        "Please enter your request body",
    ];
    pub static ref MAIN_MENU_OPTIONS: [&'static str; 5] = [
        "Build and run a new cURL command\n  \n",
        "Build and run a new wget command\n  \n",
        "Build/send new custom HTTP request\n  \n",
        "View my stored API keys\n  \n",
        "View or execute my saved commands\n  \n",
    ];
    pub static ref RESPONSE_MENU_OPTIONS: [&'static str; 3] = [
        "Write to file?\n \n",
        "View response headers\n \n",
        "View response body\n \n",
    ];
    pub static ref API_KEY_MENU_OPTIONS: [&'static str; 3] = [
        "Add a new key\n \n",
        "View my keys\n \n",
        "Delete a key\n \n",
    ];
    pub static ref SAVED_COMMAND_OPTIONS: [&'static str; 3] = [
        "Add a new saved command\n \n",
        "View my saved commands\n \n",
        "Delete a saved command\n \n",
    ];
}
