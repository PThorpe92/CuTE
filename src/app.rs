use crate::database::db;
use crate::database::db::DB;
use crate::display::AppOptions;
use crate::request::command::{CmdOpts, CMD};
use crate::request::curl::Curl;
use crate::screens::screen::{determine_line_size, Screen};
use crate::Config;
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
    pub selected: Option<usize>,
    pub command: Option<Box<dyn CMD>>,
    pub opts: Vec<AppOptions>,
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
        let base_config_dir = config_dir().unwrap();
        let config_dir = base_config_dir.join("CuTE").join("config.toml");
        let toml_str = std::fs::read_to_string(config_dir).unwrap();
        let app_config: Config = toml::from_str(&toml_str).unwrap_or_default();
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
            db: Some(Box::new(DB::new().unwrap())),
        }
    }
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_command(&mut self, command: Box<dyn CMD>) {
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
        let db = self.db.as_ref().unwrap();
        let keys = db.get_keys().unwrap();
        let mut saved_keys = Vec::new();
        for key in keys {
            saved_keys.push(format!("{}", key));
        }
        Ok(saved_keys)
    }

    pub fn add_saved_key(&mut self, key: String) -> Result<(), rusqlite::Error> {
        match self.db.as_mut().unwrap().add_key(&key) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => Err(e),
        }
    }

    pub fn remove_app_option(&mut self, opt: &AppOptions) {
        match opt {
            AppOptions::URL(_) => {
                self.command.as_mut().unwrap().set_url("");
            }
            AppOptions::Headers(_) => {
                self.command
                    .as_mut()
                    .unwrap()
                    .remove_headers(opt.get_value());
            }
            AppOptions::Outfile(_) => {
                self.command.as_mut().unwrap().set_outfile("");
            }
            AppOptions::Auth(_) => {
                self.command
                    .as_mut()
                    .unwrap()
                    .set_auth(crate::request::curl::AuthKind::None);
            }
            AppOptions::UnixSocket(_) => {
                self.command.as_mut().unwrap().set_unix_socket("");
            }
            AppOptions::EnableHeaders => {
                self.command
                    .as_mut()
                    .unwrap()
                    .enable_response_headers(false);
            }
            AppOptions::ProgressBar => {
                self.command.as_mut().unwrap().enable_progress_bar(false);
            }
            AppOptions::FailOnError => {
                self.command.as_mut().unwrap().set_fail_on_error(false);
            }
            AppOptions::Verbose => {
                self.command.as_mut().unwrap().set_verbose(false);
            }
            AppOptions::Response(_) => {
                self.command.as_mut().unwrap().set_response("");
            }
            AppOptions::SaveCommand => {
                self.command.as_mut().unwrap().save_command(false);
            }
            AppOptions::SaveToken => {
                self.command.as_mut().unwrap().save_token(false);
            }
            AppOptions::FollowRedirects => {
                self.command.as_mut().unwrap().set_follow_redirects(false);
            }
            AppOptions::UnrestrictedAuth => {
                self.command.as_mut().unwrap().set_unrestricted_auth(false);
            }
            AppOptions::TcpKeepAlive => {
                self.command.as_mut().unwrap().set_tcp_keepalive(false);
            }
            AppOptions::ProxyTunnel => {
                self.command.as_mut().unwrap().set_proxy_tunnel(false);
            }
            AppOptions::CertInfo => {
                self.command.as_mut().unwrap().set_cert_info(false);
            }
            AppOptions::MatchWildcard => {
                self.command.as_mut().unwrap().match_wildcard(false);
            }
            AppOptions::CaPath(_) => {
                self.command.as_mut().unwrap().set_ca_path("");
            }
            AppOptions::MaxRedirects(_) => {
                self.command.as_mut().unwrap().set_max_redirects(0);
            }
            AppOptions::Cookie(_) => {
                self.command
                    .as_mut()
                    .unwrap()
                    .remove_headers(opt.get_value());
            }
            AppOptions::UserAgent(_) => {
                self.command.as_mut().unwrap().set_user_agent("");
            }
            AppOptions::Referrer(_) => {
                self.command.as_mut().unwrap().set_referrer("");
            }
            AppOptions::RecDownload(_) => {
                self.command.as_mut().unwrap().set_rec_download_level(0);
            }
        }
        self.opts
            .retain(|x| mem::discriminant(x) != mem::discriminant(&opt));
    }

    // Need a button to reset everything
    pub fn remove_all_app_options(&mut self) {
        self.opts.clear();
        self.command = Some(Box::new(Curl::new()));
    }

    pub fn has_app_option(&self, opt: &AppOptions) -> bool {
        for element in self.opts.iter() {
            if mem::discriminant(opt) == mem::discriminant(element) {
                return true;
            }
        }
        false
    }

    fn should_add_option(&self, opt: &AppOptions) -> bool {
        match opt {
            // Header are the only item that should be added multiple times
            AppOptions::Headers(_) => true,
            _ => !self.has_app_option(opt),
        }
    }

    pub fn set_response(&mut self, response: String) {
        self.response = Some(response.clone());
        self.command.as_mut().unwrap().set_response(&response);
    }

    fn should_toggle(&self, opt: &AppOptions) -> bool {
        match opt {
            // Any displayOpt with no value should be toggled (bool)
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
            | AppOptions::EnableHeaders => true,
            _ => false,
        }
    }

    fn toggle_app_option(&mut self, opt: AppOptions) {
        if self.has_app_option(&opt) {
            self.remove_app_option(&opt);
            return;
        }
        match opt {
            // these are the options that work with both commands
            AppOptions::URL(_) => {
                self.command.as_mut().unwrap().set_url("");
                return;
            }
            AppOptions::RecDownload(_) => {
                self.command.as_mut().unwrap().set_rec_download_level(0);
                return;
            }
            AppOptions::Outfile(_) => {
                self.command.as_mut().unwrap().set_outfile("");
                return;
            }
            AppOptions::Verbose => self.command.as_mut().unwrap().set_verbose(true),
            AppOptions::EnableHeaders => {
                self.command.as_mut().unwrap().enable_response_headers(true)
            }
            AppOptions::ProgressBar => self.command.as_mut().unwrap().enable_progress_bar(true),
            AppOptions::FailOnError => self.command.as_mut().unwrap().set_fail_on_error(true),
            AppOptions::MatchWildcard => self.command.as_mut().unwrap().match_wildcard(true),
            AppOptions::CertInfo => self.command.as_mut().unwrap().set_cert_info(true),
            AppOptions::ProxyTunnel => self.command.as_mut().unwrap().set_proxy_tunnel(true),
            AppOptions::SaveCommand => self.command.as_mut().unwrap().save_command(true),
            AppOptions::FollowRedirects => {
                self.command.as_mut().unwrap().set_follow_redirects(true)
            }
            AppOptions::UnrestrictedAuth => {
                self.command.as_mut().unwrap().set_unrestricted_auth(true)
            }
            AppOptions::TcpKeepAlive => self.command.as_mut().unwrap().set_tcp_keepalive(true),
            AppOptions::SaveToken => self.command.as_mut().unwrap().save_token(true),
            _ => {}
        }
    }

    pub fn add_app_option(&mut self, opt: AppOptions) {
        if self.should_toggle(&opt) {
            self.toggle_app_option(opt);
            return;
        }

        if self.should_add_option(&opt) {
            self.opts.push(opt.clone());
            match opt {
                AppOptions::UnixSocket(socket) => {
                    self.command.as_mut().unwrap().set_unix_socket(&socket)
                }
                AppOptions::Headers(value) => self.command.as_mut().unwrap().add_headers(value),
                AppOptions::URL(url) => self.command.as_mut().unwrap().set_url(&url),
                AppOptions::Outfile(outfile) => {
                    self.command.as_mut().unwrap().set_outfile(&outfile)
                }
                AppOptions::Cookie(cookie) => self.command.as_mut().unwrap().add_cookie(cookie),
                AppOptions::RecDownload(i) => {
                    self.command.as_mut().unwrap().set_rec_download_level(i);
                }
                AppOptions::Response(resp) => {
                    self.command.as_mut().unwrap().set_response(&resp);
                }
                AppOptions::Referrer(referrer) => {
                    self.command.as_mut().unwrap().set_referrer(&referrer);
                }
                AppOptions::UserAgent(agent) => {
                    self.command.as_mut().unwrap().set_user_agent(&agent);
                }
                AppOptions::CaPath(ca_path) => {
                    self.command.as_mut().unwrap().set_ca_path(&ca_path);
                }
                AppOptions::MaxRedirects(max_redirects) => {
                    self.command
                        .as_mut()
                        .unwrap()
                        .set_max_redirects(max_redirects);
                }
                _ => {}
            }
        } else {
            // We Should Replace An Option, so we iterate over all the opts and replace the value
            // with the new value.
            self.handle_replace(opt.clone());
        }
    }
    fn handle_replace(&mut self, mut opt: AppOptions) {
        for option in self.opts.iter_mut() {
            match option {
                AppOptions::URL(_) => {
                    if let AppOptions::URL(ref url) = opt {
                        self.command.as_mut().unwrap().set_url(&url);
                        option.replace_value(url.clone());
                    }
                }
                AppOptions::Outfile(_) => {
                    if let AppOptions::Outfile(ref outfile) = opt {
                        option.replace_value(outfile.clone());
                        self.command.as_mut().unwrap().set_outfile(&outfile);
                    }
                }
                AppOptions::Response(_) => {
                    if let AppOptions::Response(ref response) = opt {
                        option.replace_value(opt.clone().get_value());
                        self.command.as_mut().unwrap().set_response(&response);
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
                        self.command.as_mut().unwrap().set_user_agent(&agent);
                    }
                }
                AppOptions::Referrer(_) => {
                    if let AppOptions::Referrer(ref referrer) = opt {
                        option.replace_value(String::from(referrer));
                        self.command.as_mut().unwrap().set_referrer(&referrer);
                    }
                }
                AppOptions::Cookie(_) => {
                    if let AppOptions::Cookie(ref mut cookie) = opt {
                        option.replace_value(String::from(cookie.clone()));
                        self.command.as_mut().unwrap().add_cookie(cookie.clone());
                    }
                }
                AppOptions::CaPath(_) => {
                    if let AppOptions::CaPath(ref ca_path) = opt {
                        option.replace_value(String::from(ca_path));
                        self.command.as_mut().unwrap().set_ca_path(&ca_path);
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
                        self.command.as_mut().unwrap().set_unix_socket(&socket);
                    }
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::App;
    use crate::display::AppOptions;
    use crate::request::command::{Cmd, CmdOpts};
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
}
