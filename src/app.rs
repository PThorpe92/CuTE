use crate::database::db::{SavedCommand, DB};
use crate::display::menuopts::OPTION_PADDING_MID;
use crate::display::AppOptions;
use crate::request::curl::Curl;
use crate::request::ExecuteOption;
use crate::screens::screen::Screen;
use crate::Config;
use arboard::Clipboard;
use std::io::Write;
use std::ops::DerefMut;
use std::{error, mem};
use tui::widgets::{ListItem, ListState};
use tui_input::Input;
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

/// Application.
pub struct App<'a> {
    /// toml config file
    pub config: Config,
    /// is the app running
    pub running: bool,
    /// cursor position (veritcal)
    pub cursor: usize,
    /// current screen (stack.pop)
    pub current_screen: Screen,
    /// screen stack
    pub screen_stack: Vec<Screen>,
    /// index of selected item
    pub selected: Option<usize>,
    /// command (curl or wget)
    pub command: Curl,
    /// vec of applicable options
    pub opts: Vec<AppOptions>,
    /// Input struct for tui_input dependency
    pub input: Input,
    /// vec for user input to push into
    pub messages: Vec<String>,
    /// input mode (normal or editing)
    pub input_mode: InputMode,
    /// vec of list items to select from
    pub items: Vec<ListItem<'a>>,
    /// list state for tui
    pub state: Option<ListState>,
    /// http response from executed command
    pub response: Option<String>,
    /// database connection
    pub db: Box<DB>,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self {
            config: Config::default(),
            running: true,
            cursor: 0,
            screen_stack: vec![Screen::Home],
            selected: None,
            command: Curl::default(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            opts: Vec::new(),
            items: Screen::Home.get_opts(None),
            input: Input::default(),
            state: None,
            current_screen: Screen::Home,
            response: None,
            db: Box::new(DB::new().unwrap()),
        }
    }
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_command(&mut self, command: Curl) {
        self.command = command;
    }
    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }
    pub fn redraw(&mut self) {
        if self.selected.is_some() {
            let selected = (self.selected, self.cursor);
            let current = self.current_screen.clone();
            self.goto_screen(&current);
            self.state.as_mut().unwrap().select(selected.0);
            self.cursor = selected.1;
        }
    }

    pub fn copy_to_clipboard_from_response(&self) -> Result<(), String> {
        if let Some(resp) = self.response.as_ref() {
            if let Ok(mut clipboard) = Clipboard::new() {
                if let Err(e) = clipboard.set_text(resp) {
                    return Err(e.to_string());
                }
                Ok(())
            } else {
                Err("Failed to copy to clipboard".to_string())
            }
        } else {
            Err("No response to copy".to_string())
        }
    }

    pub fn copy_to_clipboard_from_command(&mut self) -> Result<(), String> {
        if let Ok(mut clipboard) = Clipboard::new() {
            if let Err(e) = clipboard.set_text(self.command.get_command_string()) {
                return Err(e.to_string());
            }
            Ok(())
        } else {
            Err("Failed to copy to clipboard".to_string())
        }
    }

    pub fn goto_screen(&mut self, screen: &Screen) {
        self.input.reset();
        self.current_screen = screen.clone();
        self.screen_stack.push(screen.clone());
        self.cursor = 0;
        match screen {
            Screen::Method => {
                // If The Method Screen Is Hit, We Reset options
                self.clear_all_options();
                self.input.reset();
                self.items = screen.get_opts(None);
            }
            Screen::SavedKeys(_) => {
                self.items = self
                    .db
                    .as_ref()
                    .get_keys()
                    .unwrap_or_default()
                    .iter()
                    .map(|key| ListItem::new(format!("{}{}", key, OPTION_PADDING_MID)))
                    .collect();
                self.selected = None;
                return;
            }
            Screen::SavedCommands(col_name) => {
                self.items = self
                    .db
                    .as_ref()
                    .get_commands(*col_name)
                    .unwrap_or_default()
                    .iter()
                    .map(|cmd| {
                        ListItem::new(format!("{}{}", cmd.get_command(), OPTION_PADDING_MID))
                    })
                    .collect();
                self.selected = None;
                return;
            }
            Screen::ViewSavedCollections => {
                self.items = self
                    .db
                    .as_ref()
                    .get_collections()
                    .unwrap_or_default()
                    .iter()
                    .map(|col| ListItem::new(format!("{}{}", col.get_name(), OPTION_PADDING_MID)))
                    .collect();
            }
            Screen::RequestMenu(opt) if opt.as_ref().is_some_and(|op| !op.is_error()) => {
                self.input_mode = InputMode::Editing;
                self.selected = None;
            }
            _ => {
                self.items = screen.get_opts(None);
            }
        }
        self.selected = None;
    }

    pub fn go_back_screen(&mut self) {
        let last = self.screen_stack.pop().unwrap_or_default(); // current screen
        match self.screen_stack.last().cloned() {
            Some(screen) if std::mem::discriminant(&screen) == std::mem::discriminant(&last) => {
                self.go_back_screen()
            }
            Some(
                Screen::InputMenu(_)
                | Screen::CmdMenu(_)
                | Screen::ColMenu(_)
                | Screen::KeysMenu(_),
            ) => self.go_back_screen(),
            Some(Screen::RequestBodyInput) => self.goto_screen(&Screen::Method),
            Some(Screen::Error(_)) => self.goto_screen(&Screen::Home),
            Some(Screen::RequestMenu(_)) => {
                self.goto_screen(&Screen::RequestMenu(None));
            }
            Some(Screen::Method) => self.goto_screen(&Screen::Home),
            Some(screen) => {
                self.goto_screen(&screen);
            }
            _ => self.goto_screen(&Screen::Home),
        }
    }

    pub fn quit(&mut self) {
        std::io::stdout()
            .write_all(self.response.as_ref().unwrap_or(&String::new()).as_bytes())
            .unwrap();
        std::io::stdout().flush().unwrap();
        self.running = false;
    }

    pub fn get_request_body(&self) -> Option<String> {
        self.opts.iter().find_map(|opt| match opt {
            AppOptions::RequestBody(body) => Some(body.clone()),
            _ => None,
        })
    }

    pub fn move_cursor_down(&mut self) {
        match self.current_screen {
            Screen::RequestMenu(ref opt) => {
                if opt.clone().is_some_and(|op| !op.is_error()) {
                    self.goto_screen(&Screen::RequestMenu(None));
                    return;
                }
                if !self.items.is_empty() && self.cursor < self.items.len() - 1 {
                    self.cursor += 1;
                }
            }
            _ => {
                if self.items.is_empty() {
                    return;
                }
                if !self.items.is_empty() && self.cursor < self.items.len() - 1 {
                    self.cursor += 1;
                }
            }
        }
    }

    pub fn move_cursor_up(&mut self) {
        match self.current_screen {
            Screen::RequestMenu(ref opt) => {
                if opt.clone().is_some_and(|op| !op.is_error()) {
                    self.goto_screen(&Screen::RequestMenu(None));
                    return;
                }
                if let Some(res) = self.cursor.checked_sub(1) {
                    self.cursor = res;
                }
            }
            _ => {
                if self.items.is_empty() {
                    return;
                }
                if let Some(res) = self.cursor.checked_sub(1) {
                    self.cursor = res;
                }
            }
        }
    }

    pub fn set_app_input(&mut self, input: &str) {
        input.to_string().chars().for_each(|c| {
            if self
                .input
                .handle(tui_input::InputRequest::InsertChar(c))
                .is_some()
            {}
        });
    }

    pub fn get_database_items(&self) -> Option<Vec<String>> {
        match self.current_screen {
            Screen::SavedKeys(_) => Some(
                self.db
                    .as_ref()
                    .get_keys()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            ),
            Screen::SavedCommands(coll_id) => Some(
                self.db
                    .as_ref()
                    .get_commands(coll_id)
                    .unwrap_or_default()
                    .into_iter()
                    .map(|x| format!("{:?}", x))
                    .collect::<Vec<String>>(),
            ),
            Screen::ViewSavedCollections => Some(
                self.db
                    .as_ref()
                    .get_collections()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|x| x.get_name().to_string())
                    .collect::<Vec<String>>(),
            ),
            Screen::SavedCollections(_) => Some(
                self.db
                    .as_ref()
                    .get_collections()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|x| x.get_name().to_string())
                    .collect::<Vec<String>>(),
            ),
            _ => None,
        }
    }

    pub fn select_item(&mut self) {
        if let Some(state) = self.state.as_mut() {
            if let Some(selected) = state.selected() {
                self.selected = Some(selected);
            }
        }
    }

    pub fn execute_command(&mut self) -> Result<(), String> {
        let opts = &self.opts;
        self.command
            .execute(Some(Box::new(self.db.deref_mut())), opts.as_slice())
    }

    pub fn import_postman_collection(
        &mut self,
        path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let collection: Result<crate::database::postman::PostmanCollection, String> =
            serde_json::from_reader(file).map_err(|e| e.to_string());
        match collection {
            Ok(collection) => {
                let name = collection.info.name.clone();
                let cmds: Vec<SavedCommand> = collection.into();
                self.db.add_collection(&name, cmds.as_slice())
            }
            Err(e) => Err(e.into()),
        }
    }

    // Takes an array index of the selected item
    pub fn execute_saved_command(&mut self, json: &str) {
        let mut command: Curl = serde_json::from_str(json)
            .map_err(|e| e.to_string())
            .unwrap();
        let opts = &self.opts;
        command.easy_from_opts(opts.as_slice());
        match command.execute(None, opts.as_slice()) {
            Ok(_) => self.set_response(&command.get_response().unwrap_or("".to_string())),
            Err(e) => self.set_response(&e),
        };
    }

    pub fn copy_to_clipboard(&self, opt: &str) -> Result<(), String> {
        if let Ok(mut clipboard) = Clipboard::new() {
            if let Err(e) = clipboard.set_text(opt) {
                return Err(e.to_string());
            }
            Ok(())
        } else {
            Err("Failed to copy to clipboard".to_string())
        }
    }

    pub fn get_from_clipboard(&self) -> String {
        if let Ok(mut clipboard) = Clipboard::new() {
            clipboard.get_text().unwrap_or_default()
        } else {
            String::new()
        }
    }

    pub fn delete_item(&mut self, ind: i32) -> Result<(), rusqlite::Error> {
        match self.current_screen {
            Screen::CmdMenu(_) => self.db.as_ref().delete_command(ind),
            Screen::KeysMenu(_) => self.db.as_ref().delete_key(ind),
            Screen::ViewSavedCollections => self.db.as_ref().delete_collection(ind),
            _ => Ok(()),
        }
    }

    pub fn remove_app_option(&mut self, opt: &AppOptions) {
        self.command.remove_option(opt);
        self.opts
            .retain(|x| mem::discriminant(x) != mem::discriminant(opt));
    }

    pub fn clear_all_options(&mut self) {
        self.opts.clear();
        self.messages.clear();
        self.response = None;
    }

    fn has_app_option(&self, opt: &AppOptions) -> bool {
        self.opts
            .iter()
            .any(|x| mem::discriminant(x) == mem::discriminant(opt))
    }

    fn should_add_option(&self, opt: &AppOptions) -> bool {
        match opt {
            opt if opt.should_append() => true,
            _ => !self.has_app_option(opt),
        }
    }

    pub fn set_response(&mut self, response: &str) {
        self.response = Some(response.to_string());
        self.command.set_response(response);
    }

    fn toggle_app_option(&mut self, opt: AppOptions) {
        if self.has_app_option(&opt) {
            self.remove_app_option(&opt);
            self.redraw();
            return;
        }
        if opt.should_toggle() {
            self.opts.push(opt.clone());
            self.command.add_option(&opt);
        }
        self.redraw();
    }

    pub fn add_app_option(&mut self, opt: AppOptions) {
        if opt.should_toggle() {
            self.toggle_app_option(opt);
            return;
        }
        if self.should_add_option(&opt) {
            self.opts.push(opt.clone());
            self.command.add_option(&opt);
        } else {
            self.handle_replace(opt.clone());
        }
        self.selected = None;
    }

    fn handle_replace(&mut self, mut opt: AppOptions) {
        for option in self.opts.iter_mut() {
            match option {
                AppOptions::URL(_) => {
                    if let AppOptions::URL(ref url) = opt {
                        self.command.set_url(url);
                        option.replace_value(url.clone());
                    }
                }
                AppOptions::Outfile(_) => {
                    if let AppOptions::Outfile(ref outfile) = opt {
                        option.replace_value(outfile.clone());
                        self.command.set_outfile(outfile);
                    }
                }
                AppOptions::Response(_) => {
                    if let AppOptions::Response(ref response) = opt {
                        option.replace_value(opt.clone().get_value());
                        self.command.set_response(response);
                    }
                }
                AppOptions::Auth(_) => {} // This is handled by the screen
                AppOptions::UserAgent(_) => {
                    if let AppOptions::UserAgent(ref agent) = opt {
                        option.replace_value(String::from(agent));
                        self.command.set_user_agent(agent);
                    }
                }
                AppOptions::Referrer(_) => {
                    if let AppOptions::Referrer(ref referrer) = opt {
                        option.replace_value(String::from(referrer));
                        self.command.set_referrer(referrer);
                    }
                }
                AppOptions::CookiePath(_) => {
                    if let AppOptions::CookiePath(ref mut cookie) = opt {
                        option.replace_value(cookie.clone());
                        self.command.add_cookie(cookie);
                    }
                }
                AppOptions::CookieJar(_) => {
                    if let AppOptions::CookieJar(ref mut cookie) = opt {
                        option.replace_value(cookie.clone());
                        self.command.set_cookie_jar(cookie);
                    }
                }
                AppOptions::CaPath(_) => {
                    if let AppOptions::CaPath(ref ca_path) = opt {
                        option.replace_value(String::from(ca_path));
                        self.command.set_ca_path(ca_path);
                    }
                }
                AppOptions::MaxRedirects(_) => {
                    if let AppOptions::MaxRedirects(ref max_redirects) = opt {
                        option.replace_value(max_redirects.to_string());
                        self.command.set_max_redirects(*max_redirects);
                    }
                }
                AppOptions::UnixSocket(_) => {
                    if let AppOptions::UnixSocket(ref mut socket) = opt {
                        option.replace_value(socket.clone());
                        self.command.set_unix_socket(socket);
                    }
                }
                AppOptions::RequestBody(_) => {
                    if let AppOptions::RequestBody(ref mut body) = opt {
                        option.replace_value(body.clone());
                        self.command.set_request_body(body);
                    }
                }
                _ => {}
            }
        }
    }
}
