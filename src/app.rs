use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub cursor: usize,
    pub selected: Option<Item>,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub value: &'static str,      // Name of selection
    pub index: usize,             // Index of selection on the screen
    pub screen: usize,            // Which screen our selection will bring us to
    pub children: Vec<Box<Item>>, // Children of the selection
}
impl Item {
    pub fn new(value: &'static str, index: usize, screen: usize) -> Self {
        Self {
            value,
            index,
            screen,
            children: Vec::new(),
        }
    }
    pub fn add_child(&mut self, child: Item) {
        self.children.push(Box::new(child));
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            cursor: 0,
            selected: None,
            items: Vec::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn move_cursor_down(&mut self) {
        if let Some(res) = self.cursor.checked_add(1) {
            self.cursor = res;
        }
    }

    pub fn move_cursor_up(&mut self) {
        if let Some(res) = self.cursor.checked_sub(1) {
            self.cursor = res;
        }
    }

    pub fn remove_selection(&mut self) {
        if let Some(selection) = &self.selected {
            self.items.remove(selection.index);
        }
    }

    pub fn select_item(&mut self) {
        self.selected = Some(self.items[self.cursor].clone());
    }
}
