use super::{
    curl::{AuthKind, Curl},
    wget::Wget,
};
use crate::database::db::DB;

pub enum Cmd<'a> {
    Curl(Curl<'a>),
    Wget(Wget),
}

pub trait CMD: CurlOpts + CmdOpts {}

impl<'a> CMD for Cmd<'a> {}

impl<'a> CmdOpts for Cmd<'a> {
    fn execute(&mut self, db: Option<&mut Box<DB>>) -> Result<(), String> {
        match self {
            Cmd::Curl(curl) => curl.execute(db),
            Cmd::Wget(wget) => wget.execute(db),
        }
    }
    fn add_basic_auth(&mut self, info: &str) {
        match self {
            Cmd::Curl(curl) => curl.add_basic_auth(info),
            _ => {}
        }
    }
    fn get_url(&self) -> String {
        match self {
            Cmd::Curl(curl) => curl.get_url(),
            Cmd::Wget(wget) => wget.get_url(),
        }
    }
    fn set_outfile(&mut self, file: &str) {
        match self {
            Cmd::Curl(curl) => curl.set_outfile(file),
            Cmd::Wget(wget) => wget.set_outfile(file),
        }
    }
    fn get_response(&self) -> String {
        match self {
            Cmd::Curl(curl) => curl.get_response(),
            Cmd::Wget(wget) => wget.get_response(),
        }
    }
    fn set_rec_download_level(&mut self, level: usize) {
        match self {
            Cmd::Curl(curl) => curl.set_rec_download_level(level),
            Cmd::Wget(wget) => wget.set_rec_download_level(level),
        }
    }
    fn set_url(&mut self, url: &str) {
        match self {
            Cmd::Curl(curl) => curl.set_url(url),
            Cmd::Wget(wget) => wget.set_url(url),
        }
    }
    fn set_response(&mut self, response: &str) {
        match self {
            Cmd::Curl(curl) => curl.set_response(response),
            Cmd::Wget(wget) => wget.set_response(response),
        }
    }
    fn get_command_string(&mut self) -> String {
        match self {
            Cmd::Curl(curl) => curl.get_command_string(),
            Cmd::Wget(wget) => wget.get_command_string(),
        }
    }
}

impl<'a> CurlOpts for Cmd<'a> {
    fn add_cookie(&mut self, cookie: String) {
        match self {
            Cmd::Curl(curl) => curl.add_cookie(cookie),
            _ => {}
        }
    }
    fn set_follow_redirects(&mut self, opt: bool) {
        match self {
            Cmd::Curl(curl) => curl.set_follow_redirects(opt),
            _ => {}
        }
    }
    fn set_proxy_tunnel(&mut self, opt: bool) {
        match self {
            Cmd::Curl(curl) => curl.set_proxy_tunnel(opt),
            _ => {}
        }
    }
    fn match_wildcard(&mut self, opt: bool) {
        match self {
            Cmd::Curl(curl) => curl.match_wildcard(opt),
            _ => {}
        }
    }
    fn write_output(&mut self) -> Result<(), std::io::Error> {
        match self {
            Cmd::Curl(curl) => curl.write_output(),
            _ => Ok(()),
        }
    }
    fn set_method(&mut self, method: String) {
        match self {
            Cmd::Curl(curl) => curl.set_method(method),
            _ => {}
        }
    }
    fn set_auth(&mut self, auth: AuthKind) {
        match self {
            Cmd::Curl(curl) => curl.set_auth(auth),
            _ => {}
        }
    }
    fn add_headers(&mut self, headers: String) {
        match self {
            Cmd::Curl(curl) => curl.add_headers(headers),
            _ => {}
        }
    }
    fn enable_response_headers(&mut self, opt: bool) {
        match self {
            Cmd::Curl(curl) => curl.enable_response_headers(opt),
            _ => {}
        }
    }
    fn enable_progress_bar(&mut self, opt: bool) {
        match self {
            Cmd::Curl(curl) => curl.enable_progress_bar(opt),
            _ => {}
        }
    }
    fn set_cert_info(&mut self, opt: bool) {
        match self {
            Cmd::Curl(curl) => curl.set_cert_info(opt),
            _ => {}
        }
    }
    fn set_fail_on_error(&mut self, opt: bool) {
        match self {
            Cmd::Curl(curl) => curl.set_fail_on_error(opt),
            _ => {}
        }
    }
    fn save_command(&mut self, opt: bool) {
        match self {
            Cmd::Curl(curl) => curl.save_command(opt),
            _ => {}
        }
    }
    fn set_headers(&mut self, headers: Vec<String>) {
        match self {
            Cmd::Curl(curl) => curl.set_headers(headers),
            _ => {}
        }
    }
    fn set_verbose(&mut self, opt: bool) {
        match self {
            Cmd::Curl(curl) => curl.set_verbose(opt),
            _ => {}
        }
    }
    fn set_unix_socket(&mut self, socket: &str) {
        match self {
            Cmd::Curl(curl) => curl.set_unix_socket(socket),
            _ => {}
        }
    }
    fn save_token(&mut self, opt: bool) {
        match self {
            Cmd::Curl(curl) => curl.save_token(opt),
            _ => {}
        }
    }
    fn get_token(&self) -> Option<String> {
        match self {
            Cmd::Curl(curl) => curl.get_token(),
            _ => None,
        }
    }
    fn remove_headers(&mut self, headers: String) {
        match self {
            Cmd::Curl(curl) => curl.remove_headers(headers),
            _ => {}
        }
    }
    fn will_save_command(&self) -> bool {
        match self {
            Cmd::Curl(curl) => curl.will_save_command(),
            _ => false,
        }
    }
    fn set_tcp_keepalive(&mut self, opt: bool) {
        match self {
            Cmd::Curl(curl) => curl.set_tcp_keepalive(opt),
            _ => {}
        }
    }
    fn set_unrestricted_auth(&mut self, opt: bool) {
        match self {
            Cmd::Curl(curl) => curl.set_unrestricted_auth(opt),
            _ => {}
        }
    }
    fn set_referrer(&mut self, referrer: &str) {
        match self {
            Cmd::Curl(curl) => curl.set_referrer(referrer),
            _ => {}
        }
    }
    fn set_max_redirects(&mut self, redirects: usize) {
        match self {
            Cmd::Curl(curl) => curl.set_max_redirects(redirects),
            _ => {}
        }
    }
    fn set_ca_path(&mut self, path: &str) {
        match self {
            Cmd::Curl(curl) => curl.set_ca_path(path),
            _ => {}
        }
    }
    fn set_user_agent(&mut self, ua: &str) {
        match self {
            Cmd::Curl(curl) => curl.set_user_agent(ua),
            _ => {}
        }
    }
}

pub trait CmdOpts {
    fn execute(&mut self, db: Option<&mut Box<DB>>) -> Result<(), String>;
    fn add_basic_auth(&mut self, info: &str);
    fn get_url(&self) -> String;
    fn set_outfile(&mut self, file: &str);
    fn get_response(&self) -> String;
    fn set_rec_download_level(&mut self, level: usize);
    fn set_url(&mut self, url: &str);
    fn set_response(&mut self, response: &str);
    fn get_command_string(&mut self) -> String;
}

pub trait CurlOpts {
    fn add_cookie(&mut self, cookie: String);
    fn set_follow_redirects(&mut self, opt: bool);
    fn set_proxy_tunnel(&mut self, opt: bool);
    fn match_wildcard(&mut self, opt: bool);
    fn write_output(&mut self) -> Result<(), std::io::Error>;
    fn set_method(&mut self, method: String);
    fn set_auth(&mut self, auth: AuthKind);
    fn add_headers(&mut self, headers: String);
    fn enable_response_headers(&mut self, opt: bool);
    fn enable_progress_bar(&mut self, opt: bool);
    fn set_cert_info(&mut self, opt: bool);
    fn set_fail_on_error(&mut self, opt: bool);
    fn save_command(&mut self, opt: bool);
    fn set_headers(&mut self, headers: Vec<String>);
    fn set_verbose(&mut self, opt: bool);
    fn set_unix_socket(&mut self, socket: &str);
    fn save_token(&mut self, opt: bool);
    fn get_token(&self) -> Option<String>;
    fn remove_headers(&mut self, headers: String);
    fn will_save_command(&self) -> bool;
    fn set_tcp_keepalive(&mut self, opt: bool);
    fn set_unrestricted_auth(&mut self, opt: bool);
    fn set_referrer(&mut self, referrer: &str);
    fn set_max_redirects(&mut self, redirects: usize);
    fn set_ca_path(&mut self, path: &str);
    fn set_user_agent(&mut self, ua: &str);
}
