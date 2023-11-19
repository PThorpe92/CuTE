use super::{
    curl::{AuthKind, Curl},
    wget::Wget,
};
use crate::{database::db::DB, display::HeaderKind};
use std::fmt::{Display, Error, Formatter};

pub enum Cmd<'a> {
    Curl(Curl<'a>),
    Wget(Wget),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CmdType {
    Curl,
    Wget,
}

impl Display for CmdType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            CmdType::Curl => write!(f, "HTTP Request"),
            CmdType::Wget => write!(f, "Download"),
        }
    }
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
        if let Cmd::Curl(curl) = self {
            curl.add_basic_auth(info);
        }
    }

    fn has_auth(&self) -> bool {
        if let Cmd::Curl(curl) = self {
            curl.has_auth()
        } else {
            false
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
    fn add_cookie(&mut self, cookie: &str) {
        if let Cmd::Curl(curl) = self {
            curl.add_cookie(cookie);
        }
    }
    fn has_unix_socket(&self) -> bool {
        if let Cmd::Curl(curl) = self {
            curl.has_unix_socket()
        } else {
            false
        }
    }
    fn set_content_header(&mut self, kind: HeaderKind) {
        if let Cmd::Curl(curl) = self {
            curl.set_content_header(kind);
        }
    }
    fn set_upload_file(&mut self, file: &str) {
        if let Cmd::Curl(curl) = self {
            curl.set_upload_file(file);
        }
    }
    fn set_follow_redirects(&mut self, opt: bool) {
        if let Cmd::Curl(curl) = self {
            curl.set_follow_redirects(opt);
        }
    }
    fn set_proxy_tunnel(&mut self, opt: bool) {
        if let Cmd::Curl(curl) = self {
            curl.set_proxy_tunnel(opt);
        }
    }
    fn set_request_body(&mut self, body: &str) {
        if let Cmd::Curl(curl) = self {
            curl.set_request_body(body);
        }
    }
    fn match_wildcard(&mut self, opt: bool) {
        if let Cmd::Curl(curl) = self {
            curl.match_wildcard(opt);
        }
    }
    fn write_output(&mut self) -> Result<(), std::io::Error> {
        if let Cmd::Curl(curl) = self {
            curl.write_output()
        } else {
            Ok(())
        }
    }
    fn set_method(&mut self, method: String) {
        if let Cmd::Curl(curl) = self {
            curl.set_method(method);
        }
    }
    fn set_auth(&mut self, auth: AuthKind) {
        if let Cmd::Curl(curl) = self {
            curl.set_auth(auth);
        }
    }
    fn add_headers(&mut self, headers: String) {
        if let Cmd::Curl(curl) = self {
            curl.add_headers(headers);
        }
    }
    fn enable_response_headers(&mut self, opt: bool) {
        if let Cmd::Curl(curl) = self {
            curl.enable_response_headers(opt);
        }
    }
    fn enable_progress_bar(&mut self, opt: bool) {
        if let Cmd::Curl(curl) = self {
            curl.enable_progress_bar(opt);
        }
    }
    fn set_cert_info(&mut self, opt: bool) {
        if let Cmd::Curl(curl) = self {
            curl.set_cert_info(opt);
        }
    }
    fn set_fail_on_error(&mut self, opt: bool) {
        if let Cmd::Curl(curl) = self {
            curl.set_fail_on_error(opt);
        }
    }
    fn save_command(&mut self, opt: bool) {
        if let Cmd::Curl(curl) = self {
            curl.save_command(opt);
        }
    }
    fn set_verbose(&mut self, opt: bool) {
        if let Cmd::Curl(curl) = self {
            curl.set_verbose(opt);
        }
    }
    fn set_unix_socket(&mut self, socket: &str) {
        if let Cmd::Curl(curl) = self {
            curl.set_unix_socket(socket);
        }
    }
    fn save_token(&mut self, opt: bool) {
        if let Cmd::Curl(curl) = self {
            curl.save_token(opt);
        }
    }
    fn get_token(&self) -> Option<String> {
        if let Cmd::Curl(curl) = self {
            curl.get_token()
        } else {
            None
        }
    }
    fn remove_headers(&mut self, headers: String) {
        if let Cmd::Curl(curl) = self {
            curl.remove_headers(headers);
        }
    }
    fn will_save_command(&self) -> bool {
        if let Cmd::Curl(curl) = self {
            curl.will_save_command()
        } else {
            false
        }
    }
    fn set_tcp_keepalive(&mut self, opt: bool) {
        if let Cmd::Curl(curl) = self {
            curl.set_tcp_keepalive(opt);
        }
    }
    fn set_unrestricted_auth(&mut self, opt: bool) {
        if let Cmd::Curl(curl) = self {
            curl.set_unrestricted_auth(opt);
        }
    }
    fn set_referrer(&mut self, referrer: &str) {
        if let Cmd::Curl(curl) = self {
            curl.set_referrer(referrer);
        }
    }
    fn set_max_redirects(&mut self, redirects: usize) {
        if let Cmd::Curl(curl) = self {
            curl.set_max_redirects(redirects);
        }
    }
    fn set_ca_path(&mut self, path: &str) {
        if let Cmd::Curl(curl) = self {
            curl.set_ca_path(path);
        }
    }
    fn set_user_agent(&mut self, ua: &str) {
        if let Cmd::Curl(curl) = self {
            curl.set_user_agent(ua);
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
    fn has_auth(&self) -> bool;
}
pub trait CurlOpts {
    fn set_content_header(&mut self, kind: HeaderKind);
    fn set_request_body(&mut self, body: &str);
    fn set_upload_file(&mut self, file: &str);
    fn add_cookie(&mut self, cookie: &str);
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
    fn has_unix_socket(&self) -> bool;
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
