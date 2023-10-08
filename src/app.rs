use std::{error, mem};

use crate::display::DisplayOpts;
use crate::request::command::Command;
use crate::request::curl::Curl;
use crate::screens::screen::{determine_line_size, Screen};
use crate::{database::db::DB, request::response::Response};
use std::ops::{Deref, DerefMut};
use tui::widgets::{ListItem, ListState};
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
    pub response: Option<String>,
    pub db: Option<Box<DB>>,
}

impl<'a> Deref for App<'a> {
    type Target = Command<'a>;
    fn deref(&self) -> &Self::Target {
        self.command.as_ref().unwrap()
    }
}

impl<'a> DerefMut for App<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.command.as_mut().unwrap()
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
            items: Screen::Home.get_opts(None),
            input: Input::default(),
            state: None,
            current_screen: Screen::Home,
            response: None,
            db: None,
        }
    }
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_command(&mut self, command: Command<'a>) {
        self.command = Some(command);
    }

    pub fn tick(&self) {}

    pub fn goto_screen(&mut self, screen: Screen) {
        self.screen_stack.push(screen.clone());
        self.current_screen = screen.clone();

        self.cursor = 0;
        match screen {
            Screen::SavedKeys => {
                self.items = self
                    .get_saved_keys()
                    .unwrap_or(vec![])
                    .iter()
                    .map(|key| ListItem::new(format!("{}{}", key, determine_line_size())))
                    .collect();
                self.selected = None;
                return;
            }
            Screen::SavedCommands => {
                self.items = self
                    .get_saved_command_strings()
                    .unwrap_or(vec![])
                    .iter()
                    .map(|cmd| ListItem::new(format!("{}{}", cmd, determine_line_size())))
                    .collect();
                self.selected = None;
                return;
            }
            _ => {
                self.items = screen.get_opts(None);
            }
        }
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
                    self.goto_screen(screen.clone());
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
        // Lorenzo: I fixed a bug here with -1, where the cursor would roll off the screen.
        if self.items.is_empty() || self.cursor >= self.items.len() - 1 {
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

    pub fn get_response_headers(&mut self) -> String {
        if let Ok(response) = Response::from_raw_string(&self.response.as_ref().unwrap().clone()) {
            response.get_headers().to_string()
        } else {
            return String::from("No headers found");
        }
    }

    pub fn execute_command(&mut self) -> Result<(), String> {
        match self.command.as_mut().unwrap() {
            Command::Curl(ref mut curl) => {
                // continue lazy loading by only opening connection if we need to
                if curl.will_save_command() && self.db.is_none() {
                    self.db = Some(Box::new(DB::new().unwrap()));
                }
                match curl.execute(&mut self.db) {
                    Ok(_) => {
                        self.response = curl.get_response();
                        Ok(())
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
            Command::Wget(wget) => wget.execute(),
        }
    }

    pub fn get_saved_command_strings(&mut self) -> Result<Vec<String>, String> {
        if self.db.is_none() {
            self.db = Some(Box::new(DB::new().unwrap()));
        }
        let db = self.db.as_ref().unwrap();
        let commands = db.get_commands().unwrap();
        let saved_commands = commands.iter().map(|cmd| cmd.get_command()).collect();
        Ok(saved_commands)
    }

    pub fn get_saved_command_json(&mut self) -> Result<Vec<String>, String> {
        if self.db.is_none() {
            self.db = Some(Box::new(DB::new().unwrap()));
        }
        let commands = self.db.as_ref().unwrap().get_commands().unwrap();
        let saved_commands = commands
            .iter()
            .map(|cmd| cmd.get_curl_json())
            .collect::<Vec<String>>();
        Ok(saved_commands)
    }

    pub fn execute_saved_command(&mut self, index: usize) {
        let saved_commands = self.get_saved_command_json().unwrap();
        let _ = saved_commands.iter().map(|x| x.clone());
        let json = saved_commands.get(index).unwrap();
        let mut command: Curl = serde_json::from_str(json.as_str()).unwrap();
        match command.execute(&mut None) {
            Ok(_) => self.set_response(
                command
                    .get_response()
                    .unwrap_or("Command failed to execute".to_string())
                    .clone(),
            ),
            Err(e) => self.set_response(e.to_string()),
        };
        self.goto_screen(Screen::Response(self.response.clone().unwrap()));
    }

    pub fn delete_saved_command(&mut self, index: usize) {
        let saved_commands = self.get_saved_command_strings().unwrap();
        let command = saved_commands.get(index).unwrap();
        self.db.as_mut().unwrap().delete_command(command).unwrap();
        self.goto_screen(Screen::SavedCommands);
    }

    pub fn delete_saved_key(&mut self, index: usize) {
        let saved_keys = self.get_saved_keys().unwrap();
        let key = saved_keys.get(index).unwrap();
        self.db.as_mut().unwrap().delete_key(key).unwrap();
        self.goto_screen(Screen::SavedKeys);
    }

    pub fn delete_item(&mut self, index: usize) {
        match self.current_screen {
            Screen::SavedCommands => self.delete_saved_command(index),
            Screen::SavedKeys => self.delete_saved_key(index),
            _ => {}
        }
    }

    pub fn get_saved_keys(&mut self) -> Result<Vec<String>, String> {
        if self.db.is_none() {
            self.db = Some(Box::new(DB::new().unwrap()));
        }
        let db = self.db.as_ref().unwrap();
        let keys = db.get_keys().unwrap();
        let mut saved_keys = Vec::new();
        for key in keys {
            saved_keys.push(format!("{}", key));
        }
        Ok(saved_keys)
    }

    pub fn add_saved_key(&mut self, key: String) -> Result<(), rusqlite::Error> {
        if self.db.is_none() {
            self.db = Some(Box::new(DB::new().unwrap()));
        }
        match self.db.as_mut().unwrap().add_key(&key) {
            Ok(_) => {
                println!("Key Added");
                return Ok(());
            }
            Err(_) => {
                println!("OOPS");
                Ok(())
            }
        }
    }

    pub fn remove_display_option(&mut self, opt: &DisplayOpts) {
        for option in self.opts.iter() {
            match option {
                DisplayOpts::Verbose => {
                    self.command.as_mut().unwrap().set_verbose();
                }
                DisplayOpts::SaveCommand => {
                    self.command.as_mut().unwrap().save_command(false);
                }
                DisplayOpts::SaveToken => match self.command.as_mut().unwrap() {
                    Command::Curl(curl) => curl.save_token(false),
                    _ => {}
                },
                DisplayOpts::URL(_) => self.command.as_mut().unwrap().set_url(""),
                DisplayOpts::Headers(..) => self
                    .command
                    .as_mut()
                    .unwrap()
                    .remove_headers(vec![opt.get_value()]),
                DisplayOpts::Outfile(_) => self.command.as_mut().unwrap().set_outfile(""),
                DisplayOpts::Response(_) => self.response = None,
                DisplayOpts::RecDownload(_) => {
                    self.command.as_mut().unwrap().set_rec_download_level(0)
                }
                DisplayOpts::Auth(_) => self
                    .command
                    .as_mut()
                    .unwrap()
                    .set_auth(crate::request::curl::AuthKind::None),
            }
        }
        self.opts
            .retain(|x| std::mem::discriminant(x) != std::mem::discriminant(opt));
    }

    pub fn remove_all_display_options(&mut self) {
        self.opts.clear();
    }
    // Display option is some state that requires us to display the users
    // current selection on the screen so they know what they have selected
    // Lorenzo - Changing this because I dont think its doing what I want it to do.
    pub fn has_display_option(&self, opt: &DisplayOpts) -> bool {
        for element in self.opts.iter() {
            // I only care if its the same KIND of option, not the same value
            // This is annoying, I tried to do this an easier way

            match *element {
                DisplayOpts::URL(_) => {
                    if mem::discriminant(opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::Headers(_) => {
                    if mem::discriminant(opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::Outfile(_) => {
                    if mem::discriminant(opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::Response(_) => {
                    if mem::discriminant(opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::SaveCommand => {
                    if mem::discriminant(opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::Verbose => {
                    if mem::discriminant(opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::RecDownload(_) => {
                    if mem::discriminant(opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::Auth(_) => {
                    if mem::discriminant(opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::SaveToken => {
                    if mem::discriminant(opt) == mem::discriminant(element) {
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
    fn should_add_option(&self, opt: &DisplayOpts) -> bool {
        // Lets match the type of display option
        // We know that only 1 URL should ever be added,
        // So if we're adding a URL we should replace it if it already exists
        match opt {
            DisplayOpts::URL(_) => !self.has_display_option(opt), // URL should be replaced if exists
            DisplayOpts::Headers(_) => true, // Headers should be "pushed" or added
            DisplayOpts::Outfile(_) => !self.has_display_option(opt), // Outfile should be replaced
            DisplayOpts::Verbose => !self.has_display_option(opt), // Verbose should be toggled
            DisplayOpts::SaveCommand => !self.has_display_option(opt), // Save command should be toggled
            DisplayOpts::Response(_) => !self.has_display_option(opt), // Response should be replaced
            DisplayOpts::RecDownload(_) => !self.has_display_option(opt), // Recursive download depth should be replaced
            DisplayOpts::Auth(_) => !self.has_display_option(opt),        // Auth should be replaced
            DisplayOpts::SaveToken => !self.has_display_option(opt), // Save token should be toggled
        }
    }

    pub fn set_response(&mut self, response: String) {
        self.response = Some(response.clone());
        if let Some(cmd) = &mut self.command {
            cmd.set_response(&response);
        }
    }

    fn should_toggle(&self, opt: &DisplayOpts) -> bool {
        match opt {
            DisplayOpts::Verbose => true,
            DisplayOpts::SaveCommand => true,
            DisplayOpts::SaveToken => true,
            _ => false,
        }
    }

    pub fn toggle_display_option(&mut self, opt: DisplayOpts) {
        if self.has_display_option(&opt) {
            self.remove_display_option(&opt);
            return;
        }
        let will_save_cmd = self.will_save_command();
        match opt.clone() {
            DisplayOpts::Verbose => {
                self.command.as_mut().unwrap().set_verbose();
            }
            DisplayOpts::SaveCommand => {
                self.save_command(!will_save_cmd);
                println!("Saving Command: {}", !will_save_cmd);
            }
            DisplayOpts::SaveToken => self.save_token(),
            _ => {}
        }
        self.opts.push(opt.clone());
    }

    pub fn add_display_option(&mut self, opt: DisplayOpts) {
        // We either add the option or we replace the existing one

        // first we look and see if its an option we are going to toggle..
        if self.should_toggle(&opt) {
            self.toggle_display_option(opt);
            return;
        }

        if self.should_add_option(&opt) {
            self.opts.push(opt.clone());
            match opt {
                DisplayOpts::Headers((key, value)) => {
                    // Push Header To Shareable Command
                    self.command
                        .as_mut()
                        .unwrap()
                        .add_headers(vec![format!("{}:{}", key, value)]);
                }
                DisplayOpts::URL(url) => {
                    self.set_url(&url);
                }
                DisplayOpts::Outfile(outfile) => {
                    self.command.as_mut().unwrap().set_outfile(&outfile);
                }

                _ => {
                    // Nothing
                }
            }
        } else {
            // We Should Replace An Option

            for option in self.opts.iter_mut() {
                match option {
                    DisplayOpts::URL(_) => {
                        option.replace_value(opt.get_value());
                        self.command.as_mut().unwrap().set_url(&opt.get_value());
                    }
                    DisplayOpts::Headers(..) => option.replace_value(opt.get_value()),
                    DisplayOpts::Outfile(_) => option.replace_value(opt.get_value()),
                    DisplayOpts::Response(_) => option.replace_value(opt.get_value()),
                    DisplayOpts::RecDownload(_) => option.replace_value(opt.get_value()),
                    DisplayOpts::Auth(_) => option.replace_value(opt.get_value()),
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::App;
    use crate::display::DisplayOpts;
    use crate::request::command::Command;
    use crate::request::curl::Curl;

    // helper return app instance with curl command
    fn return_app_cmd() -> App<'static> {
        let mut app = App::default();
        app.set_command(Command::Curl(Curl::new()));
        app
    }

    #[test]
    fn test_store_key() {
        let mut app = return_app_cmd();
        let token = "abcdefghijklmnop".to_string();
        let _ = app.add_saved_key(token.clone());
        assert!(
            app.db
                .as_ref()
                .unwrap()
                .get_keys()
                .unwrap()
                .iter()
                .filter(|x| x.is_key(&token))
                .collect::<Vec<_>>()
                .len()
                > 0
        );
    }

    #[test]
    fn test_toggle_verbose() {
        let mut app = return_app_cmd();
        // Add one.
        app.add_display_option(crate::display::DisplayOpts::Verbose);
        assert!(app.has_display_option(&DisplayOpts::Verbose));
        // this should toggle
        app.add_display_option(DisplayOpts::Verbose);
        assert!(!app.has_display_option(&DisplayOpts::Verbose));
    }

    #[test]
    fn test_replace_display_opt() {
        let mut app = return_app_cmd();
        let url = "https://www.google.com".to_string();
        app.add_display_option(DisplayOpts::URL(url.clone()));
        assert!(app.command.as_ref().unwrap().get_url() == url);
        // overwrite the url
        let new_url = "https://www.github.com".to_string();
        app.add_display_option(DisplayOpts::URL(new_url.clone()));
        assert!(app.command.as_ref().unwrap().get_url() == new_url);
    }

    #[test]
    fn test_response_headers() {
        let response = json!(
            {
            "headers": {
                "content-type": "application/json",
                "content-length": "123",
                "server": "nginx"
            },
            "body": "Hello World"
            }
        );
        let mut app = return_app_cmd();
        app.set_response(response.to_string());
        app.get_response_headers();
        assert!(
            app.get_response_headers()
                == "content-type: application/json\ncontent-length: 123\nserver: nginx\n"
        );
    }

    #[test]
    fn test_remove_display_option() {
        let mut app = return_app_cmd();
        let url = "https://www.google.com";
        app.add_display_option(DisplayOpts::URL(String::from(url)));
        app.remove_display_option(&DisplayOpts::URL(String::from(url)));
    }
}
