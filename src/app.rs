use std::{error, mem};

use tui::widgets::{ListItem, ListState};
use tui_input::Input;

use crate::display::displayopts::DisplayOpts;
use crate::display::shareablecmd::ShareableCommand;
use crate::request::command::Command;
use crate::screens::screen::Screen;

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
    pub response: Option<String>,
    pub shareable_command: Option<ShareableCommand>,
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
            response: None,
            shareable_command: None,
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

        // If We Are Going Into A Screen Where There Would Be A Sharable Command
        // Lets Init That
        match self.current_screen {
            Screen::RequestMenu(_) => {
                self.shareable_command = Some(ShareableCommand::new());
            }
            _ => {} // Lets Add Cases As We Add Features
        }

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
            Some(_) => match self.screen_stack.last() {
                Some(screen) => {
                    self.cursor = 0;
                    self.selected = None;
                    self.current_screen = screen.clone();
                }
                _ => {}
            },
            None => {}
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
    // Lorenzo - Changing this because I dont think its doing what I want it to do.
    pub fn has_display_option(&self, opt: DisplayOpts) -> bool {
        // this is what we were doing before - self.opts.iter().any(|x| &x == &&opt)
        // I think this is what we want to do instead
        // We want to check if the option is in the vector
        // If it is, we want to return true
        // If it is not, we want to return false
        // We can do this by iterating over the vector and checking if the option is in the vector
        for element in self.opts.iter() {
            // I only care if its the same KIND of option, not the same value
            // This is annoying, I tried to do this an easier way

            match *element {
                DisplayOpts::URL(_) => {
                    if mem::discriminant(&opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::Headers(_) => {
                    if mem::discriminant(&opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::Outfile(_) => {
                    if mem::discriminant(&opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::Response(_) => {
                    if mem::discriminant(&opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::SaveCommand => {
                    if mem::discriminant(&opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::ShareableCmd(_) => {
                    if mem::discriminant(&opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::Verbose => {
                    if mem::discriminant(&opt) == mem::discriminant(element) {
                        return true;
                    }
                }
            }
        }
        // Otherwise, its not there.
        false
    }

    // Lorenzo - Im adding this function as a slightly more
    // robust version of has_display_option, to test if we should be replacing a value or adding a new one
    fn should_replace_or_add(&self, opt: DisplayOpts) -> bool {
        // Lets match the type of display option
        // We know that only 1 URL should ever be added,
        // So if we're adding a URL we should replace it if it already exists
        match opt {
            DisplayOpts::URL(_) => !self.has_display_option(opt.clone()), // URL Should Be Replaced If It Already Exists
            DisplayOpts::Headers(_) => true, // Headers Should Be "Pushed" or Added
            DisplayOpts::Outfile(_) => !self.has_display_option(opt.clone()), // Outfile Should Be Replaced
            DisplayOpts::Verbose => !self.has_display_option(opt.clone()), // Verbose Should Be Toggled
            DisplayOpts::SaveCommand => !self.has_display_option(opt.clone()), // Save Command Should Be Toggled
            DisplayOpts::Response(_) => !self.has_display_option(opt.clone()), // Response Should Be Replaced
            DisplayOpts::ShareableCmd(_) => !self.has_display_option(opt.clone()), // Shareable Command Should Be Replaced With The New Version
            _ => false, // Anything Else Should Be Replaced.
        }
    }

    pub fn set_response(&mut self, response: String) {
        self.response = Some(response.clone());
        match self.command {
            Some(ref mut cmd) => {
                cmd.set_response(response);
            }
            None => {}
        }
    }

    // user selects once, we add. twice we remove.
    // Lorenzo - When a display option is added, we should also add it to the sharable command
    pub fn add_display_option(
        &mut self,
        mut opt: DisplayOpts, /* We Make This Mutable Because We Need To Compare It To A Mutable Reference Later */
    ) {
        if self.should_replace_or_add(opt.clone()) {
            // TRUE = We Should Add An Option
            // Adding The New Test Here. should_replace_or_add
            // The user has not yet added this command option yet, and so therefore, we push it to the opts vector.
            // I need a copy of the option before we move it
            let match_opt = opt.clone(); // Lets refactor all this later.
            self.opts.push(opt);
            // We also need to add it to the sharable command
            // but we need to know what kind of option it is
            // the best way I can think of right now is to match it but I'm sure there's a better way
            match match_opt {
                // Im cloning this to shut the borrow checker up, there is probably a better way
                DisplayOpts::Verbose => {
                    // Just add the verbose flag to the command
                    self.shareable_command.as_mut().unwrap().set_verbose(true);
                }
                DisplayOpts::Headers((key, value)) => {
                    // Push Header To Shareable Command
                    self.shareable_command
                        .as_mut()
                        .unwrap()
                        .push_header((key, value));
                }
                DisplayOpts::URL(url) => {
                    // Set URL To Sharable Command
                    self.shareable_command.as_mut().unwrap().set_url(url);
                }
                DisplayOpts::Outfile(outfile) => {
                    // Set Outfile To Shareable Command
                    self.shareable_command
                        .as_mut()
                        .unwrap()
                        .set_outfile(outfile);
                }
                _ => {
                    // Nothing
                    // This display opt does not factor into the sharable command.
                }
            }
        } else {
            // FALSE = We Should Replace An Option
            // The user has already added this command option, and so therefore, we should replace the old value with the new value.
            //self.opts.retain(|x| x != &opt);
            // Sorry, this is my way to do this idk if its the right way, but this is what makes sense to me in my head
            for element in self.opts.iter_mut() {
                // Same thing down here, I only care if its the same KIND of option, not the same value
                // Again, this is annoying, I tried to do this an easier way
                // but mem::discriminant doesnt like element as a comparison so I need to be particular
                // Sorry lets refactor this
                // TODO: Refactor This.

                // We Want To Just Replace A URL
                if let DisplayOpts::URL(_) = element {
                    *element = opt.clone(); // Copy The New URL Into The Old One
                    return;
                }

                // TODO: Headers Will Be Handled Differently.

                // TODO: Outfile Will Be Handled Differently.

                // TODO: Verbose & Save Command Will Be Handled Differently.

                // TODO: Other Shit
            }
        }
    }
}
