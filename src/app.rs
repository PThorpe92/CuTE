use std::error;
use tui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    pub cursor: usize,
    pub current_screen: Screen,
    pub selected: Option<usize>,
    pub items: Vec<ListItem<'a>>,
    pub state: Option<ListState>,
}

pub static CURL_MENU_OPTIONS: [&str; 7] = [
    "Add a URL\n \n",
    "Add Authentication\n \n",
    "Add Headers\n \n",
    "Enable verbose output\n \n",
    "Specify request output file\n \n",
    "Add Request Body\n \n",
    "Execute command\n \n",
];

pub static WGET_MENU_OPTIONS: [&str; 8] = [
    "Add a URL\n \n",
    "Add Authentication\n \n ",
    "Add Headers\n \n",
    "Enable verbose output\n \n",
    "Specify download output file\n \n",
    "Specify recursive download\n \n",
    "Add Request Body\n \n",
    "Execute command\n \n",
];

pub static COMMAND_MENU_OPTIONS: [&str; 6] = [
    "Choose an HTTP method:\n \n",
    "GET\n \n",
    "POST\n \n",
    "PUT\n \n",
    "DELETE\n \n",
    "PATCH\n \n",
];

pub static HTTP_MENU_OPTIONS: [&str; 6] = [
    "Add a URL\n \n",
    "Authentication\n \n",
    "Add Headers\n \n",
    "Specify response output file\n \n ",
    "Add Request Body\n \n",
    "Send Request \n \n",
];

pub static MAIN_MENU_OPTIONS: [&str; 5] = [
    "Build and run a new cURL command\n  \n",
    "Build and run a new wget command\n  \n",
    "Build/send new custom HTTP request\n  \n",
    "View my stored API keys\n  \n",
    "View or execute my saved commands\n  \n",
];

// TODO: keys and saved commands menus
// Filler for now until these are implemented:
// obviously we need to be fetching these from the db
pub static API_KEY_MENU_OPTIONS: [&str; 3] = [
    "Add a new key\n \n",
    "View my keys\n \n",
    "Delete a key\n \n",
];
pub static SAVED_COMMAND_OPTIONS: [&str; 3] = [
    "Add a new saved command\n \n",
    "View my saved commands\n \n",
    "Delete a saved command\n \n",
];

#[derive(Debug, PartialEq)]
pub enum Screen {
    Home,
    Command(Command),
    Curl,
    Wget,
    Custom,
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
            Screen::Curl => {
                return CURL_MENU_OPTIONS
                    .iter()
                    .map(|i| ListItem::new(*i))
                    .collect();
            }
            Screen::Wget => {
                return WGET_MENU_OPTIONS
                    .iter()
                    .map(|i| ListItem::new(*i))
                    .collect();
            }
            Screen::Custom => {
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
        }
    }

    pub fn get_list(&self) -> List<'a> {
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
            Screen::Curl => "Curl",
            Screen::Wget => "Wget",
            Screen::Custom => "Custom HTTP Request",
            Screen::Saved => "My Saved Commands",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Curl,
    Wget,
    Custom,
}
impl Command {
    pub fn default() -> Self {
        Command::Curl
    }
    pub fn to_string(&self) -> String {
        match self {
            Command::Curl => "Curl",
            Command::Wget => "Wget",
            Command::Custom => "Custom",
        }
        .to_string()
    }
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self {
            current_screen: Screen::Home,
            running: true,
            cursor: 0,
            selected: None,
            items: Vec::from(Screen::Home.get_opts()),
            state: None,
        }
    }
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

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
        // NOTES:
        // All we are doing by getting the 'state' is to be able to set the selected list item
        // but that doesn't do us any good... as a ListItem just contains some text which we can't
        // match, We really only need the index of the selected item, so by hitting the enter key,
        //we can just store the usize index of the "selected" item, and match that to decide
        // what to do next on a screen by screen basis.

        let state = self.state.as_mut().unwrap();
        if let Some(selected) = state.selected() {
            // ^^^ returns usize index
            self.selected = Some(selected);
        }
    }
}
