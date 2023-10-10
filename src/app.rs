use crate::database::db;
use crate::display::DisplayOpts;
use crate::request::command::{AppCmd, CmdOpts};
use crate::request::curl::Curl;
use crate::screens::screen::{determine_line_size, Screen};
use crate::Config;
use crate::{database::db::DB};
use dirs::config_dir;
use std::{error, mem};
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
pub struct App<'a> {
    pub config: Config,
    /// Is the application running?
    pub running: bool,
    pub cursor: usize,
    pub current_screen: Screen,
    pub screen_stack: Vec<Screen>,
    pub command: Option<AppCmd>,
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

impl<'a> Default for App<'a> {
    fn default() -> Self {
        let mut app_config = Config::default();
        let base_config_dir = config_dir().unwrap();
        let config_dir = base_config_dir.join("cute.toml");
        if let Ok(file_str) = std::fs::read_to_string(config_dir) {
            app_config = toml::from_str(&file_str).unwrap();
        }
        Self {
            config: app_config,
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
    pub fn set_command(&mut self, command: AppCmd) {
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
        self.screen_stack.pop(); // current screen
        match self.screen_stack.last() {
            Some(Screen::InputMenu(_)) => self.go_back_screen(),
            // is that recursion in prod????? o_0
            Some(screen) if screen == &self.current_screen => self.go_back_screen(),
            Some(screen) => {
                self.goto_screen(screen.clone());
            }
            None => self.goto_screen(Screen::Home),
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn move_cursor_down(&mut self) {
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

    pub fn execute_command(&mut self) -> Result<(), String> {
        if self.db.is_none() {
            self.db = Some(Box::new(db::DB::new().unwrap()));
        }
        self.command.as_mut().unwrap().execute(self.db.as_mut());
        Ok(())
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
        match command.execute(None) {
            Ok(_) => self.set_response(command.get_response().clone()),
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
                return Ok(());
            }
            Err(_) => Ok(()),
        }
    }

    pub fn remove_display_option(&mut self, opt: &DisplayOpts) {
        match self.command.as_mut().unwrap() {
            AppCmd::CurlCmd(curl) => match opt {
                DisplayOpts::URL(_) => {
                    curl.set_url("");
                }
                DisplayOpts::Headers(_) => {
                    curl.remove_headers(opt.get_value());
                }
                DisplayOpts::Outfile(_) => {
                    curl.set_outfile("");
                }
                DisplayOpts::Auth(_) => {
                    curl.set_auth(crate::request::curl::AuthKind::None);
                }
                DisplayOpts::UnixSocket(_) => {
                    curl.set_unix_socket("");
                }
                DisplayOpts::EnableHeaders => {
                    curl.enable_response_headers(false);
                }
                DisplayOpts::ProgressBar => {
                    curl.enable_progress_bar(false);
                }
                DisplayOpts::FailOnError => {
                    curl.set_fail_on_error(false);
                }
                DisplayOpts::Verbose => {
                    curl.set_verbose(false);
                }
                DisplayOpts::RecDownload(_) => {}
                DisplayOpts::Response(_) => {
                    curl.set_response("");
                }
                DisplayOpts::SaveCommand => {
                    curl.save_command(false);
                }
                DisplayOpts::SaveToken => {
                    curl.save_token(false);
                }
            },
            AppCmd::WgetCmd(wget) => match opt {
                DisplayOpts::URL(_) => {
                    wget.set_url("");
                }
                DisplayOpts::Outfile(_) => {
                    wget.set_outfile("");
                }
                DisplayOpts::RecDownload(_) => {
                    wget.set_rec_download_level(0);
                }
                _ => {}
            },
        }
    }
    // Need a button to reset everything
    pub fn remove_all_display_options(&mut self) {
        self.opts.clear();
        match self.command.as_ref().unwrap() {
            AppCmd::CurlCmd(_) => {
                self.command = Some(AppCmd::CurlCmd(Box::new(Curl::new())));
            }
            AppCmd::WgetCmd(_) => {
                self.command = Some(AppCmd::WgetCmd(Box::new(crate::request::wget::Wget::new())));
            }
        }
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
                DisplayOpts::UnixSocket(_) => {
                    if mem::discriminant(opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::EnableHeaders => {
                    if mem::discriminant(opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::ProgressBar => {
                    if mem::discriminant(opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::FailOnError => {
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
            DisplayOpts::UnixSocket(_) => !self.has_display_option(opt), // Unix socket should be replaced
            DisplayOpts::EnableHeaders => !self.has_display_option(opt), // Enable headers should be toggled
            DisplayOpts::ProgressBar => !self.has_display_option(opt), // Progress bar should be toggled
            DisplayOpts::FailOnError => !self.has_display_option(opt),
        }
    }

    pub fn set_response(&mut self, response: String) {
        self.response = Some(response.clone());
        self.command.as_mut().unwrap().set_response(&response);
    }

    fn should_toggle(&self, opt: &DisplayOpts) -> bool {
        match opt {
            DisplayOpts::Verbose
            | DisplayOpts::FailOnError
            | DisplayOpts::ProgressBar
            | DisplayOpts::SaveCommand
            | DisplayOpts::SaveToken
            | DisplayOpts::EnableHeaders => true,
            _ => false,
        }
    }

    pub fn toggle_display_option(&mut self, opt: DisplayOpts) {
        if self.has_display_option(&opt) {
            self.remove_display_option(&opt);
            return;
        }
        match opt.clone() {
            DisplayOpts::URL(_) => {
                self.command.as_mut().unwrap().set_url("");
                return;
            }
            DisplayOpts::RecDownload(_) => {
                self.command.as_mut().unwrap().set_rec_download_level(0);
                return;
            }
            DisplayOpts::Outfile(_) => {
                self.command.as_mut().unwrap().set_outfile("");
                return;
            }
            _ => {}
        }
        match self.command.as_ref().unwrap() {
            AppCmd::CurlCmd(curl) => {
                let will_save_cmd = curl.will_save_command();
                match opt.clone() {
                    DisplayOpts::Verbose => match self.command.as_mut().unwrap() {
                        AppCmd::CurlCmd(curl) => {
                            curl.set_verbose(true);
                        }
                        _ => {}
                    },
                    DisplayOpts::EnableHeaders => match self.command.as_mut().unwrap() {
                        AppCmd::CurlCmd(curl) => {
                            curl.enable_response_headers(true);
                        }
                        _ => {}
                    },
                    DisplayOpts::ProgressBar => match self.command.as_mut().unwrap() {
                        AppCmd::CurlCmd(curl) => {
                            curl.enable_progress_bar(true);
                        }
                        _ => {}
                    },
                    DisplayOpts::SaveCommand => {
                        match self.command.as_mut().unwrap() {
                            AppCmd::CurlCmd(curl) => {
                                curl.save_command(will_save_cmd);
                            }
                            _ => {}
                        };
                    }
                    DisplayOpts::SaveToken => {
                        match self.command.as_mut().unwrap() {
                            AppCmd::CurlCmd(curl) => {
                                curl.save_token(true);
                            }
                            _ => {}
                        };
                    }
                    DisplayOpts::FailOnError => {
                        match self.command.as_mut().unwrap() {
                            AppCmd::CurlCmd(curl) => {
                                curl.set_fail_on_error(true);
                            }
                            _ => {}
                        };
                    }
                    _ => {}
                }
            }
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
            match self.command.as_mut().unwrap() {
                AppCmd::CurlCmd(curl) => match opt {
                    DisplayOpts::UnixSocket(socket) => curl.set_unix_socket(&socket),
                    DisplayOpts::Headers(value) => curl.add_headers(value),
                    DisplayOpts::URL(url) => curl.set_url(&url),
                    DisplayOpts::Auth(_) => {}
                    DisplayOpts::FailOnError => curl.set_fail_on_error(true),
                    DisplayOpts::EnableHeaders => curl.enable_response_headers(true),
                    DisplayOpts::ProgressBar => curl.enable_progress_bar(true),
                    DisplayOpts::Outfile(outfile) => curl.set_outfile(&outfile),
                    DisplayOpts::RecDownload(_) => {}
                    DisplayOpts::Response(_) => {}
                    _ => {}
                },
                AppCmd::WgetCmd(wget) => match opt {
                    DisplayOpts::URL(url) => wget.set_url(&url),
                    DisplayOpts::Outfile(outfile) => wget.set_outfile(&outfile),
                    DisplayOpts::RecDownload(level) => wget.set_rec_download_level(level),
                    DisplayOpts::Response(_) => {}
                    _ => {}
                },
            }
        } else {
            // We Should Replace An Option, so we iterate over all the opts and replace the value
            // with the new value.
            for option in self.opts.iter_mut() {
                match option {
                    DisplayOpts::URL(_) => {
                        if let DisplayOpts::URL(ref url) = opt {
                            option.replace_value(url.clone());
                        }
                    }
                    DisplayOpts::Outfile(_) => {
                        if let DisplayOpts::Outfile(ref outfile) = opt {
                            option.replace_value(outfile.clone());
                            self.command.as_mut().unwrap().set_outfile(&outfile);
                        }
                    }
                    DisplayOpts::Response(_) => {
                        if let DisplayOpts::Response(ref response) = opt {
                            option.replace_value(opt.clone().get_value());
                            self.command.as_mut().unwrap().set_response(&response);
                        }
                    }
                    DisplayOpts::SaveCommand => {
                        if let DisplayOpts::SaveCommand = opt {
                            *option = DisplayOpts::SaveCommand;
                        }
                    }
                    DisplayOpts::RecDownload(_) => {
                        if let DisplayOpts::RecDownload(level) = opt {
                            option.replace_value(level.to_string());
                        }
                    }
                    DisplayOpts::Auth(_) => {
                        if let DisplayOpts::Auth(ref auth) = opt {
                            option.replace_value(String::from(auth));
                        }
                    }
                    DisplayOpts::UnixSocket(_) => {
                        if let DisplayOpts::UnixSocket(ref socket) = opt {
                            *option = DisplayOpts::UnixSocket(String::from(socket));
                            match self.command.as_mut().unwrap() {
                                AppCmd::CurlCmd(curl) => {
                                    curl.set_unix_socket(&socket);
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::App;
    use crate::display::DisplayOpts;
    use crate::request::command::{AppCmd, CmdOpts};
    use crate::request::curl::Curl;

    // helper return app instance with curl command
    fn return_app_cmd() -> App<'static> {
        let mut app = App::default();
        app.set_command(AppCmd::CurlCmd(Box::new(Curl::new())));
        app
    }

    #[test]
    fn test_add_display_option() {
        let mut app = return_app_cmd();
        let url = "https://www.google.com";
        app.add_display_option(DisplayOpts::URL(String::from(url)));
        assert!(app.command.as_ref().unwrap().get_url() == url);
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
    fn test_remove_display_option() {
        let mut app = return_app_cmd();
        let url = "https://www.google.com";
        app.add_display_option(DisplayOpts::URL(String::from(url)));
        app.remove_display_option(&DisplayOpts::URL(String::from(url)));
    }
}
