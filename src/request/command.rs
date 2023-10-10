use super::curl::AuthKind;
use crate::database::db::DB;

pub enum AppCmd {
    CurlCmd(Box<dyn CombinedOpts>),
    WgetCmd(Box<dyn CmdOpts>),
}

pub trait CombinedOpts: CmdOpts + CurlOpts {}

impl CmdOpts for AppCmd {
    fn execute(&mut self, db: Option<&mut Box<DB>>) -> Result<(), String> {
        match self {
            AppCmd::CurlCmd(opts) => opts.execute(db),
            AppCmd::WgetCmd(opts) => opts.execute(db),
        }
    }
    fn add_basic_auth(&mut self, info: &str) {
        match self {
            AppCmd::CurlCmd(opts) => opts.add_basic_auth(info),
            AppCmd::WgetCmd(opts) => opts.add_basic_auth(info),
        }
    }
    fn get_url(&self) -> String {
        match self {
            AppCmd::CurlCmd(opts) => opts.get_url(),
            AppCmd::WgetCmd(opts) => opts.get_url(),
        }
    }
    fn set_outfile(&mut self, file: &str) {
        match self {
            AppCmd::CurlCmd(opts) => opts.set_outfile(file),
            AppCmd::WgetCmd(opts) => opts.set_outfile(file),
        }
    }
    fn get_response(&self) -> String {
        match self {
            AppCmd::CurlCmd(opts) => opts.get_response(),
            AppCmd::WgetCmd(opts) => opts.get_response(),
        }
    }
    fn set_rec_download_level(&mut self, level: usize) {
        match self {
            AppCmd::CurlCmd(_) => {}
            AppCmd::WgetCmd(opts) => opts.set_rec_download_level(level),
        }
    }
    fn set_url(&mut self, url: &str) {
        match self {
            AppCmd::CurlCmd(opts) => opts.set_url(url),
            AppCmd::WgetCmd(opts) => opts.set_url(url),
        }
    }
    fn set_response(&mut self, response: &str) {
        match self {
            AppCmd::CurlCmd(opts) => opts.set_response(response),
            AppCmd::WgetCmd(opts) => opts.set_response(response),
        }
    }
    fn get_command_string(&mut self) -> String {
        match self {
            AppCmd::CurlCmd(opts) => opts.get_command_string(),
            AppCmd::WgetCmd(opts) => opts.get_command_string(),
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
    fn write_output(&mut self) -> Result<(), std::io::Error>;
    fn set_method(&mut self, method: String);
    fn set_auth(&mut self, auth: AuthKind);
    fn add_headers(&mut self, headers: String);
    fn enable_response_headers(&mut self, opt: bool);
    fn enable_progress_bar(&mut self, opt: bool);
    fn set_fail_on_error(&mut self, opt: bool);
    fn save_command(&mut self, opt: bool);
    fn set_headers(&mut self, headers: Vec<String>);
    fn set_verbose(&mut self, opt: bool);
    fn set_unix_socket(&mut self, socket: &str);
    fn save_token(&mut self, opt: bool);
    fn get_token(&self) -> Option<String>;
    fn remove_headers(&mut self, headers: String);
    fn will_save_command(&self) -> bool;
}
