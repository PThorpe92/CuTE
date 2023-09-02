use std::error;
use tui::widgets::{ListItem, ListState};
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    pub cursor: usize,
    pub current_screen: Screen,
    pub selected: Option<ListItem<'a>>,
    pub items: Vec<ListItem<'a>>,
    pub state: Option<ListState>,
}

#[derive(Debug, PartialEq)]
pub enum Screen {
    Home,
    Command(Command),
    Keys,
    Saved,
}

impl Screen {
    pub fn default() -> Self {
        Screen::Home
    }
    pub fn to_string(&self) -> String {
        match self {
            Screen::Home => "Home",
            Screen::Command(_) => "Command",
            Screen::Keys => "Keys",
            Screen::Saved => "Saved",
        }
        .to_string()
    }
}

#[derive(Debug, PartialEq)]
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
            items: vec![],
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
        if let Some(item) = self.items.get_mut(self.cursor).cloned() {
            self.selected = Some(item)
        } else {
            self.selected = None
        }
    }
}
