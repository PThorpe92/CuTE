use crate::database::db::{SavedCollection, SavedCommand, SavedKey, DB};
use crate::display::menuopts::OPTION_PADDING_MID;
use crate::display::{AppOptions, HeaderKind};
use crate::request::curl::{AuthKind, Curl};
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
    pub command: Curl<'a>,
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
    pub fn set_command(&mut self, command: Curl<'a>) {
        self.command = command;
    }
    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }
    pub fn tick(&self) {}

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
        self.screen_stack.push(screen.clone());
        self.current_screen = screen.clone();

        self.cursor = 0;
        match screen {
            Screen::Method => {
                // If The Method Screen Is Hit, We Reset options
                self.remove_all_app_options();
                self.input.reset();
                self.items = screen.get_opts(None);
            }
            Screen::SavedKeys => {
                self.items = self
                    .get_saved_keys()
                    .unwrap_or_default()
                    .iter()
                    .map(|key| ListItem::new(format!("{}{}", key, OPTION_PADDING_MID)))
                    .collect();
                self.selected = None;
                return;
            }
            Screen::SavedCommands(col_name) => {
                self.items = self
                    .get_saved_commands(*col_name)
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
                    .get_collections()
                    .unwrap_or_default()
                    .iter()
                    .map(|col| ListItem::new(format!("{}{}", col.name, OPTION_PADDING_MID)))
                    .collect();
            }
            _ => {
                self.items = screen.get_opts(None);
            }
        }
        self.selected = None;
    }

    pub fn go_back_screen(&mut self) {
        let last = self.screen_stack.pop().unwrap_or_default(); // current screen
        match self.screen_stack.last() {
            Some(screen) if screen == &last => self.go_back_screen(),
            Some(
                Screen::InputMenu(_)
                | Screen::CmdMenu(_)
                | Screen::ColMenu(_)
                | Screen::KeysMenu(_),
            ) => self.go_back_screen(),
            Some(Screen::RequestBodyInput) => self.goto_screen(&Screen::Method),
            Some(Screen::RequestMenu(ref e)) => {
                if e.to_lowercase().contains("error") || e.to_lowercase().contains("alert") {
                    self.goto_screen(&Screen::RequestMenu(String::new()));
                } else {
                    self.goto_screen(&Screen::Method);
                }
            }
            Some(screen) => {
                self.goto_screen(&screen.clone());
            }
            _ => self.goto_screen(&Screen::Home),
        }
    }

    pub fn quit(&mut self) {
        std::io::stdout()
            .write_all(self.get_response().as_bytes())
            .unwrap();
        // make sure the response is flushed to stdout
        std::io::stdout().flush().unwrap();
        self.running = false;
    }

    pub fn move_cursor_down(&mut self) {
        if self.items.is_empty() {
            return;
        }
        if !self.items.is_empty() && self.cursor < self.items.len() - 1 {
            self.cursor += 1;
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

    pub fn set_app_input(&mut self, input: &str) {
        input.to_string().chars().for_each(|c| {
            if self
                .input
                .handle(tui_input::InputRequest::InsertChar(c))
                .is_some()
            {}
        });
    }

    pub fn get_url(&self) -> &str {
        self.command.get_url()
    }

    pub fn select_item(&mut self) {
        if let Some(state) = self.state.as_mut() {
            if let Some(selected) = state.selected() {
                self.selected = Some(selected);
            }
        }
    }

    pub fn execute_command(&mut self) -> Result<(), String> {
        self.command.execute(Some(Box::new(self.db.deref_mut())))
    }

    pub fn get_saved_keys(&self) -> Result<Vec<SavedKey>, rusqlite::Error> {
        self.db.as_ref().get_keys()
    }

    pub fn get_collections(&self) -> Result<Vec<SavedCollection>, rusqlite::Error> {
        self.db.get_collections()
    }

    pub fn add_saved_key(&mut self, key: String) -> Result<(), rusqlite::Error> {
        match self.db.as_ref().add_key(&key) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn get_saved_commands(
        &self,
        col_name: Option<i32>,
    ) -> Result<Vec<SavedCommand>, rusqlite::Error> {
        self.db.as_ref().get_commands(col_name)
    }

    pub fn create_postman_collection(&mut self, name: &str) -> Result<(), rusqlite::Error> {
        self.db.create_collection(name)
    }

    #[rustfmt::skip]
    pub fn import_postman_collection(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let collection: Result<crate::database::postman::PostmanCollection, String> =
            serde_json::from_reader(file).map_err(|e| e.to_string());
        if let Ok(collection) = collection {
            let name = collection.info.name.clone();
            let cmds: Vec<SavedCommand> = collection.into();
            self.db.add_collection(&name, cmds.as_slice())
        } else {
            Err("Failed to import collection".into())
        }
    }

    // Takes an array index of the selected item
    pub fn execute_saved_command(&mut self, index: usize) {
        if let Ok(saved_commands) = self.get_saved_commands(None) {
            match saved_commands.get(index) {
                Some(cmd) => {
                    let mut command: Curl = serde_json::from_str(cmd.get_curl_json())
                        .map_err(|e| e.to_string())
                        .unwrap();
                    command.easy_from_opts();
                    match command.execute(None) {
                        Ok(_) => self.set_response(&command.get_response()),
                        Err(e) => self.set_response(&e),
                    };
                    self.goto_screen(&Screen::Response(self.response.clone().unwrap()));
                }
                None => self.goto_screen(&Screen::Error("Saved command not found".to_string())),
            }
        } else {
            self.goto_screen(&Screen::Error("Saved command not found".to_string()));
        }
    }

    pub fn set_key_label(&self, key: i32, label: &str) -> Result<(), String> {
        self.db.set_key_label(key, label).map_err(|e| e.to_string())
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

    pub fn get_response(&self) -> &str {
        match self.response.as_ref() {
            Some(response) => response,
            None => "",
        }
    }

    pub fn delete_saved_command(&mut self, ind: i32) -> Result<(), rusqlite::Error> {
        self.db.delete_command(ind)?;
        self.goto_screen(&Screen::SavedCommands(None));
        Ok(())
    }

    pub fn has_auth(&self) -> bool {
        self.command.has_auth()
    }

    pub fn has_unix_socket(&self) -> bool {
        self.command.has_unix_socket()
    }

    pub fn has_url(&self) -> bool {
        !self.command.get_url().is_empty()
    }

    pub fn delete_saved_key(&mut self, index: i32) -> Result<(), rusqlite::Error> {
        self.db.as_ref().delete_key(index)?;
        self.goto_screen(&Screen::SavedKeys);
        Ok(())
    }

    pub fn delete_collection(&mut self, id: i32) -> Result<(), rusqlite::Error> {
        self.db.delete_collection(id)?;
        self.goto_screen(&Screen::SavedCommands(None));
        Ok(())
    }

    pub fn rename_collection(&mut self, id: i32, name: &str) -> Result<(), rusqlite::Error> {
        self.db.rename_collection(id, name)?;
        self.goto_screen(&Screen::SavedCollections);
        Ok(())
    }

    pub fn delete_item(&mut self, ind: i32) -> Result<(), rusqlite::Error> {
        match self.current_screen {
            Screen::CmdMenu(_) => self.delete_saved_command(ind),
            Screen::KeysMenu(_) => self.delete_saved_key(ind),
            _ => Ok(()),
        }
    }

    // I know this is a hideous slab of code but due to the enormous verbosity of all these match statments
    // this is better than the alternative
#[rustfmt::skip]
    pub fn remove_app_option(&mut self, opt: &AppOptions) {
        match opt {
            AppOptions::URL(_)             => self.command.set_url(""),
            AppOptions::Outfile(_)         => self.command.set_outfile(""),
            AppOptions::UploadFile(_)      => self.command.set_upload_file(""),
            AppOptions::UnixSocket(_)      => self.command.set_unix_socket(""),
            AppOptions::ProgressBar        => self.command.enable_progress_bar(false),
            AppOptions::FailOnError        => self.command.set_fail_on_error(false),
            AppOptions::Verbose            => self.command.set_verbose(false),
            AppOptions::Response(_)        => self.command.set_response(""),
            AppOptions::SaveCommand        => self.command.save_command(false),
            AppOptions::SaveToken          => self.command.save_token(false),
            AppOptions::FollowRedirects    => self.command.set_follow_redirects(false),
            AppOptions::UnrestrictedAuth   => self.command.set_unrestricted_auth(false),
            AppOptions::TcpKeepAlive       => self.command.set_tcp_keepalive(false),
            AppOptions::ProxyTunnel        => self.command.set_proxy_tunnel(false),
            AppOptions::CertInfo           => self.command.set_cert_info(false),
            AppOptions::MatchWildcard      => self.command.match_wildcard(false),
            AppOptions::CaPath(_)          => self.command.set_ca_path(""),
            AppOptions::MaxRedirects(_)    => self.command.set_max_redirects(0),
            AppOptions::UserAgent(_)       => self.command.set_user_agent(""),
            AppOptions::Referrer(_)        => self.command.set_referrer(""),
            AppOptions::RequestBody(_)     => self.command.set_request_body(""),
            AppOptions::Cookie(_)          => self.command.remove_headers(&opt.get_value()),
            AppOptions::Headers(_)         => self.command.remove_headers(&opt.get_value()),
            AppOptions::Auth(_)            => self.command.set_auth(crate::request::curl::AuthKind::None),
            AppOptions::EnableHeaders      => self.command.enable_response_headers(false),
            AppOptions::ContentHeaders(_)  => self.command.set_content_header(HeaderKind::None),
        }
        self.opts
            .retain(|x| mem::discriminant(x) != mem::discriminant(opt));
    }

    // Need a button to reset everything
    pub fn remove_all_app_options(&mut self) {
        self.opts.clear();
        self.messages.clear();
        self.response = None;
    }

    pub fn has_app_option(&self, opt: &AppOptions) -> bool {
        self.opts
            .iter()
            .any(|x| mem::discriminant(x) == mem::discriminant(opt))
    }

    fn should_add_option(&self, opt: &AppOptions) -> bool {
        match opt {
            // push headers, reset everything else
            AppOptions::Headers(_) => true,
            _ => !self.has_app_option(opt),
        }
    }

    pub fn set_response(&mut self, response: &str) {
        self.response = Some(response.to_string());
        self.command.set_response(response);
    }

    fn should_toggle(&self, opt: &AppOptions) -> bool {
        match opt {
            // Any Option with no string value (boolean) should be toggled
            AppOptions::Verbose
            | AppOptions::FollowRedirects
            | AppOptions::UnrestrictedAuth
            | AppOptions::TcpKeepAlive
            | AppOptions::ProxyTunnel
            | AppOptions::CertInfo
            | AppOptions::MatchWildcard
            | AppOptions::FailOnError
            | AppOptions::ProgressBar
            | AppOptions::SaveCommand
            | AppOptions::SaveToken
            | AppOptions::ContentHeaders(_) // Headers can be pushed but these are pre-defined
            | AppOptions::EnableHeaders => true,
            _ => false,
        }
    }

#[rustfmt::skip]
    fn toggle_app_option(&mut self, opt: AppOptions) {
        if self.has_app_option(&opt) {
            self.remove_app_option(&opt);
            self.redraw();
            return;
        }
        match opt {
            AppOptions::Verbose          => self.command.set_verbose(true),
            AppOptions::EnableHeaders    => self.command.enable_response_headers(true),
            AppOptions::ProgressBar      => self.command.enable_progress_bar(true),
            AppOptions::FailOnError      => self.command.set_fail_on_error(true),
            AppOptions::MatchWildcard    => self.command.match_wildcard(true),
            AppOptions::CertInfo         => self.command.set_cert_info(true),
            AppOptions::ProxyTunnel      => self.command.set_proxy_tunnel(true),
            AppOptions::SaveCommand      => self.command.save_command(true),
            AppOptions::FollowRedirects  => self.command.set_follow_redirects(true),
            AppOptions::UnrestrictedAuth => self.command.set_unrestricted_auth(true),
            AppOptions::TcpKeepAlive     => self.command.set_tcp_keepalive(true),
            AppOptions::SaveToken        => self.command.save_token(true),
            AppOptions::ContentHeaders(k)=> self.command.set_content_header(k),
            AppOptions::Auth(ref kind)   => self.command.set_auth(kind.clone()),
            // Auth will be toggled for all types except for Basic, Bearer and digest 
            _ => {}
        }
        self.opts.push(opt);
        self.redraw();
    }

#[rustfmt::skip]
    pub fn add_app_option(&mut self, opt: AppOptions) {
        if self.should_toggle(&opt) {
            self.toggle_app_option(opt);
            return;
        }

        if self.should_add_option(&opt) {
            self.opts.push(opt.clone());
            match opt.clone() {
                // other options will be set at the input menu
                // TODO: Consolidate this garbage spaghetti nonsense
                AppOptions::Auth(authkind) => match authkind {
                    AuthKind::Spnego => {
                        self.command.set_auth(authkind);
                    }
                    AuthKind::Ntlm => {
                        self.command.set_auth(authkind);
                    }
                    AuthKind::AwsSigv4 => {
                        self.command.set_auth(authkind);
                    }
                    // all auth that doesn't take input is toggled
                    _ => self.toggle_app_option(opt),
                }
                AppOptions::UnixSocket(socket) =>  self.command.set_unix_socket(&socket),

                AppOptions::Headers(value) => self.command.add_headers(&value),

                AppOptions::URL(url) => self.command.set_url(&url),

                AppOptions::Outfile(outfile) => self.command.set_outfile(&outfile),

                AppOptions::Cookie(cookie) => self.command.add_cookie(&cookie),

                AppOptions::Response(resp) => self.command.set_response(&resp),

                AppOptions::Referrer(referrer) => self.command.set_referrer(&referrer),

                AppOptions::UserAgent(agent) => self.command.set_user_agent(&agent),

                AppOptions::CaPath(ca_path) => self.command.set_ca_path(&ca_path),

                AppOptions::RequestBody(body) => self.command.set_request_body(&body),

                AppOptions::MaxRedirects(max_redirects) => self.command.set_max_redirects(max_redirects),
                _ => {}
            }
        } else {
            // We Should Replace An Option, so we iterate over all the opts and replace the value
            // with the new value.
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
                AppOptions::Cookie(_) => {
                    if let AppOptions::Cookie(ref mut cookie) = opt {
                        option.replace_value(cookie.clone());
                        self.command.add_cookie(cookie);
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
