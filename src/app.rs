use crate::database::db::{SavedCommand, SavedKey, DB};
use crate::display::menuopts::OPTION_PADDING_MID;
use crate::display::{AppOptions, HeaderKind};
use crate::request::command::CMD;
use crate::request::curl::{AuthKind, Curl};
use crate::screens::screen::Screen;
use crate::Config;
use arboard::Clipboard;
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
    pub command: Option<Box<dyn CMD>>,
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
            command: None,
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
    pub fn set_command(&mut self, command: Box<dyn CMD>) {
        self.command = Some(command);
    }
    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }
    pub fn tick(&self) {}

    pub fn redraw(&mut self) {
        if self.selected.is_some() {
            let selected = (self.selected, self.cursor);
            self.goto_screen(self.current_screen.clone());
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
        if let Some(cmd) = self.command.as_ref() {
            if let Ok(mut clipboard) = Clipboard::new() {
                if let Err(e) = clipboard.set_text(cmd.get_command_string()) {
                    return Err(e.to_string());
                }
                Ok(())
            } else {
                Err("Failed to copy to clipboard".to_string())
            }
        } else {
            Err("No command to copy".to_string())
        }
    }

    pub fn goto_screen(&mut self, screen: Screen) {
        // Push New/Next Screen Onto The Screen Stack
        self.screen_stack.push(screen.clone());

        // Set The Current Screen
        self.current_screen = screen.clone();

        self.cursor = 0;
        match screen {
            Screen::Method => {
                // If The Method Screen Is Hit, We Reset options
                self.remove_all_app_options();
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
            Screen::SavedCommands => {
                self.items = self
                    .get_saved_commands()
                    .unwrap_or_default()
                    .iter()
                    .map(|cmd| {
                        ListItem::new(format!("{}{}", cmd.get_command(), OPTION_PADDING_MID))
                    })
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
            Some(Screen::InputMenu(_)) | Some(Screen::CmdMenu(_)) | Some(Screen::KeysMenu(_)) => {
                self.go_back_screen()
            }
            // is that recursion in prod????? o_0
            Some(screen) if screen == &self.current_screen => self.go_back_screen(),
            Some(Screen::RequestMenu(_)) => {
                // This is to remove errors from the stack
                self.goto_screen(Screen::RequestMenu(String::new()));
            }
            Some(screen) => {
                self.goto_screen(screen.clone());
            }
            None => self.goto_screen(Screen::Home),
        }
    }

    pub fn quit(&mut self) {
        if let Some(resp) = self.response.as_ref() {
            let _ = std::process::Command::new("echo")
                .arg(resp)
                .spawn()
                .map_err(|e| e.to_string())
                .unwrap();
        }
        self.running = false;
    }

    pub fn move_cursor_down(&mut self) {
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

    pub fn select_item(&mut self) {
        if let Some(state) = self.state.as_mut() {
            if let Some(selected) = state.selected() {
                // ^^^ returns usize index
                self.selected = Some(selected);
            }
        }
    }

    pub fn execute_command(&mut self) -> Result<(), String> {
        self.command.as_mut().unwrap().execute(Some(&mut self.db))
    }

    pub fn get_saved_keys(&self) -> Result<Vec<SavedKey>, rusqlite::Error> {
        self.db.as_ref().get_keys()
    }

    pub fn add_saved_key(&mut self, key: String) -> Result<(), rusqlite::Error> {
        match self.db.as_ref().add_key(&key) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn get_saved_commands(&self) -> Result<Vec<SavedCommand>, rusqlite::Error> {
        self.db.as_ref().get_commands()
    }

    // Takes an array index of the selected item
    pub fn execute_saved_command(&mut self, index: usize) {
        if let Ok(saved_commands) = self.get_saved_commands() {
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
                    self.goto_screen(Screen::Response(self.response.clone().unwrap()));
                }
                None => self.goto_screen(Screen::Error("Saved command not found".to_string())),
            }
        } else {
            self.goto_screen(Screen::Error("Saved command not found".to_string()));
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
        self.db.as_mut().delete_command(ind)?;
        self.goto_screen(Screen::SavedCommands);
        Ok(())
    }

    pub fn has_auth(&self) -> bool {
        self.command.as_ref().unwrap().has_auth()
    }

    pub fn has_unix_socket(&self) -> bool {
        self.command.as_ref().unwrap().has_unix_socket()
    }

    pub fn has_url(&self) -> bool {
        !self.command.as_ref().unwrap().get_url().is_empty()
    }

    pub fn delete_saved_key(&mut self, index: i32) -> Result<(), rusqlite::Error> {
        self.db.as_ref().delete_key(index)?;
        self.goto_screen(Screen::SavedKeys);
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
            AppOptions::URL(_)           => self.command.as_mut().unwrap().set_url(""),
            AppOptions::Outfile(_)       => self.command.as_mut().unwrap().set_outfile(""),
            AppOptions::UploadFile(_)    => self.command.as_mut().unwrap().set_upload_file(""),
            AppOptions::UnixSocket(_)    => self.command.as_mut().unwrap().set_unix_socket(""),
            AppOptions::ProgressBar      => self.command.as_mut().unwrap().enable_progress_bar(false),
            AppOptions::FailOnError      => self.command.as_mut().unwrap().set_fail_on_error(false),
            AppOptions::Verbose          => self.command.as_mut().unwrap().set_verbose(false),
            AppOptions::Response(_)      => self.command.as_mut().unwrap().set_response(""),
            AppOptions::SaveCommand      => self.command.as_mut().unwrap().save_command(false),
            AppOptions::SaveToken        => self.command.as_mut().unwrap().save_token(false),
            AppOptions::FollowRedirects  => self.command.as_mut().unwrap().set_follow_redirects(false),
            AppOptions::UnrestrictedAuth => self.command.as_mut().unwrap().set_unrestricted_auth(false),
            AppOptions::TcpKeepAlive     => self.command.as_mut().unwrap().set_tcp_keepalive(false),
            AppOptions::ProxyTunnel      => self.command.as_mut().unwrap().set_proxy_tunnel(false),
            AppOptions::CertInfo         => self.command.as_mut().unwrap().set_cert_info(false),
            AppOptions::MatchWildcard    => self.command.as_mut().unwrap().match_wildcard(false),
            AppOptions::CaPath(_)        => self.command.as_mut().unwrap().set_ca_path(""),
            AppOptions::MaxRedirects(_)  => self.command.as_mut().unwrap().set_max_redirects(0),
            AppOptions::UserAgent(_)     => self.command.as_mut().unwrap().set_user_agent(""),
            AppOptions::Referrer(_)      => self.command.as_mut().unwrap().set_referrer(""),
            AppOptions::RecDownload(_)   => self.command.as_mut().unwrap().set_rec_download_level(0),
            AppOptions::RequestBody(_)   => self.command.as_mut().unwrap().set_request_body(""),
            AppOptions::Cookie(_)        => self.command.as_mut().unwrap().remove_headers(&opt.get_value()),
            AppOptions::Headers(_)       => self.command.as_mut().unwrap().remove_headers(&opt.get_value()),
            AppOptions::Auth(_)          => self.command.as_mut().unwrap().set_auth(crate::request::curl::AuthKind::None),
            AppOptions::EnableHeaders    => self.command.as_mut().unwrap().enable_response_headers(false),
            AppOptions::ContentHeaders(_)=> self.command.as_mut().unwrap().set_content_header(HeaderKind::None),
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
        if self.command.is_some() {
            self.command.as_mut().unwrap().set_response(response);
        }
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
            AppOptions::Verbose          => self.command.as_mut().unwrap().set_verbose(true),
            AppOptions::EnableHeaders    => self.command.as_mut().unwrap().enable_response_headers(true),
            AppOptions::ProgressBar      => self.command.as_mut().unwrap().enable_progress_bar(true),
            AppOptions::FailOnError      => self.command.as_mut().unwrap().set_fail_on_error(true),
            AppOptions::MatchWildcard    => self.command.as_mut().unwrap().match_wildcard(true),
            AppOptions::CertInfo         => self.command.as_mut().unwrap().set_cert_info(true),
            AppOptions::ProxyTunnel      => self.command.as_mut().unwrap().set_proxy_tunnel(true),
            AppOptions::SaveCommand      => self.command.as_mut().unwrap().save_command(true),
            AppOptions::FollowRedirects  => self.command.as_mut().unwrap().set_follow_redirects(true),
            AppOptions::UnrestrictedAuth => self.command.as_mut().unwrap().set_unrestricted_auth(true),
            AppOptions::TcpKeepAlive     => self.command.as_mut().unwrap().set_tcp_keepalive(true),
            AppOptions::SaveToken        => self.command.as_mut().unwrap().save_token(true),
            AppOptions::ContentHeaders(k)=> self.command.as_mut().unwrap().set_content_header(k),
            AppOptions::Auth(ref kind)   => self.command.as_mut().unwrap().set_auth(kind.clone()),
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
                        self.command.as_mut().unwrap().set_auth(authkind);
                    }
                    AuthKind::Ntlm => {
                        self.command.as_mut().unwrap().set_auth(authkind);
                    }
                    AuthKind::AwsSigv4 => {
                        self.command.as_mut().unwrap().set_auth(authkind);
                    }
                    // all auth that doesn't take input is toggled
                    _ => self.toggle_app_option(opt),
                }
                AppOptions::UnixSocket(socket) =>  self.command.as_mut().unwrap().set_unix_socket(&socket),

                AppOptions::Headers(value) => self.command.as_mut().unwrap().add_headers(&value),

                AppOptions::URL(url) => self.command.as_mut().unwrap().set_url(&url),

                AppOptions::Outfile(outfile) => self.command.as_mut().unwrap().set_outfile(&outfile),

                AppOptions::Cookie(cookie) => self.command.as_mut().unwrap().add_cookie(&cookie),

                AppOptions::RecDownload(i) => self.command.as_mut().unwrap().set_rec_download_level(i),

                AppOptions::Response(resp) => self.command.as_mut().unwrap().set_response(&resp),

                AppOptions::Referrer(referrer) => self.command.as_mut().unwrap().set_referrer(&referrer),

                AppOptions::UserAgent(agent) => self.command.as_mut().unwrap().set_user_agent(&agent),

                AppOptions::CaPath(ca_path) => self.command.as_mut().unwrap().set_ca_path(&ca_path),

                AppOptions::RequestBody(body) => self.command.as_mut().unwrap().set_request_body(&body),

                AppOptions::MaxRedirects(max_redirects) => self.command
                        .as_mut()
                        .unwrap()
                        .set_max_redirects(max_redirects),
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
                        self.command.as_mut().unwrap().set_url(url);
                        option.replace_value(url.clone());
                    }
                }
                AppOptions::Outfile(_) => {
                    if let AppOptions::Outfile(ref outfile) = opt {
                        option.replace_value(outfile.clone());
                        self.command.as_mut().unwrap().set_outfile(outfile);
                    }
                }
                AppOptions::Response(_) => {
                    if let AppOptions::Response(ref response) = opt {
                        option.replace_value(opt.clone().get_value());
                        self.command.as_mut().unwrap().set_response(response);
                    }
                }
                AppOptions::RecDownload(_) => {
                    if let AppOptions::RecDownload(level) = opt {
                        option.replace_value(level.to_string());
                        self.command.as_mut().unwrap().set_rec_download_level(level);
                    }
                }
                AppOptions::Auth(_) => {} // This is handled by the screen
                AppOptions::UserAgent(_) => {
                    if let AppOptions::UserAgent(ref agent) = opt {
                        option.replace_value(String::from(agent));
                        self.command.as_mut().unwrap().set_user_agent(agent);
                    }
                }
                AppOptions::Referrer(_) => {
                    if let AppOptions::Referrer(ref referrer) = opt {
                        option.replace_value(String::from(referrer));
                        self.command.as_mut().unwrap().set_referrer(referrer);
                    }
                }
                AppOptions::Cookie(_) => {
                    if let AppOptions::Cookie(ref mut cookie) = opt {
                        option.replace_value(cookie.clone());
                        self.command.as_mut().unwrap().add_cookie(cookie);
                    }
                }
                AppOptions::CaPath(_) => {
                    if let AppOptions::CaPath(ref ca_path) = opt {
                        option.replace_value(String::from(ca_path));
                        self.command.as_mut().unwrap().set_ca_path(ca_path);
                    }
                }
                AppOptions::MaxRedirects(_) => {
                    if let AppOptions::MaxRedirects(ref max_redirects) = opt {
                        option.replace_value(max_redirects.to_string());
                        self.command
                            .as_mut()
                            .unwrap()
                            .set_max_redirects(*max_redirects);
                    }
                }
                AppOptions::UnixSocket(_) => {
                    if let AppOptions::UnixSocket(ref mut socket) = opt {
                        option.replace_value(socket.clone());
                        self.command.as_mut().unwrap().set_unix_socket(socket);
                    }
                }
                AppOptions::RequestBody(_) => {
                    if let AppOptions::RequestBody(ref mut body) = opt {
                        option.replace_value(body.clone());
                        self.command.as_mut().unwrap().set_request_body(body);
                    }
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {

    /*
       use super::App;
       use crate::display::AppOptions;
       use crate::request::command::Cmd;
       use crate::request::curl::Curl;
       // helper return app instance with curl command
       fn return_app_cmd() -> App<'static> {
           let mut app = App::default();
           app.set_command(Box::new(Cmd::Curl(Curl::new())));
           app
       }


       #[test]
       fn test_add_app_option() {
           let mut app = return_app_cmd();
           let url = "https://www.google.com";
           app.add_app_option(AppOptions::URL(String::from(url)));
           assert!(app.command.as_ref().unwrap().get_url() == url);
       }

       #[test]
       fn test_toggle_verbose() {
           let mut app = return_app_cmd();
           // Add one.
           app.add_app_option(crate::display::AppOptions::Verbose);
           assert!(app.has_app_option(&AppOptions::Verbose));
           // this should toggle
           app.add_app_option(AppOptions::Verbose);
           assert!(!app.has_app_option(&AppOptions::Verbose));
       }

       #[test]
       fn test_replace_app_opt() {
           let mut app = return_app_cmd();
           let url = "https://www.google.com".to_string();
           app.add_app_option(AppOptions::URL(url.clone()));
           assert!(app.command.as_ref().unwrap().get_url() == url);
           // overwrite the url
           let new_url = "https://www.github.com".to_string();
           app.add_app_option(AppOptions::URL(new_url.clone()));
           assert!(app.command.as_ref().unwrap().get_url() == new_url);
       }

       #[test]
       fn test_remove_app_option() {
           let mut app = return_app_cmd();
           let url = "https://www.google.com";
           app.add_app_option(AppOptions::URL(String::from(url)));
           app.remove_app_option(&AppOptions::URL(String::from(url)));
       }

    */
}
