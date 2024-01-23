use crate::database::db::DB;

use super::command::CMD;

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct Wget {
    cmd: Vec<String>,
    rec_level: Option<usize>,
    auth: Option<String>,
    url: String,
    output: String,
    response: Option<String>,
}

impl CMD for Wget {
    // This is just if we want to build a string from the command. Not sure if we are going
    // to need this, but we may end up storing download commands in DB

    fn execute(&mut self, _db: Option<&mut Box<DB>>) -> Result<(), String> {
        self.build_command();
        let command = std::process::Command::new("sh")
            .arg("-c")
            .args([self.cmd.join(" ")]) // Rest Of The Command Arguments
            .output()
            .expect("failed to execute process");
        if command.status.success() {
            self.response = Some(String::from_utf8(command.stdout).unwrap());
            Ok(())
        } else {
            Err(String::from_utf8(command.stderr).unwrap())
        }
    }

    fn add_basic_auth(&mut self, info: &str) {
        let mut split = info.split(':');
        let usr = split.next().unwrap();
        let pwd = split.next().unwrap();
        self.auth = Some(format!("--user={} --password={}", usr, pwd));
    }

    fn get_url(&self) -> &str {
        self.url.as_str()
    }

    fn set_outfile(&mut self, file: &str) {
        self.output = file.to_string();
    }

    fn get_response(&self) -> String {
        self.response
            .clone()
            .unwrap_or(String::from("No download response"))
    }

    fn set_rec_download_level(&mut self, level: usize) {
        self.rec_level = Some(level);
    }

    fn set_url(&mut self, url: &str) {
        self.url = url.to_string();
    }

    fn set_response(&mut self, response: &str) {
        self.response = Some(response.to_string());
    }

    fn get_command_string(&self) -> String {
        self.cmd.join(" ").trim().to_string()
    }

    fn build_command_string(&mut self) {
        let mut cmdstr = vec![String::from("wget")];
        if self.has_url() {
            cmdstr.push(self.url.clone());
        }
        if self.has_rec() {
            cmdstr.push(format!("-r {}", self.rec_level.unwrap()));
        }
        if let Some(ref auth) = self.auth {
            cmdstr.push(auth.clone());
        }
        if self.has_output() {
            cmdstr.push(format!("-O {}", self.output));
        }
        self.cmd = cmdstr;
    }

    fn has_auth(&self) -> bool {
        self.auth.is_some()
    }
    fn set_content_header(&mut self, _kind: crate::display::HeaderKind) {}
    fn set_request_body(&mut self, _body: &str) {}
    fn set_upload_file(&mut self, _file: &str) {}
    fn set_follow_redirects(&mut self, _opt: bool) {}
    fn set_unrestricted_auth(&mut self, _opt: bool) {}
    fn set_proxy_tunnel(&mut self, _opt: bool) {}
    fn add_cookie(&mut self, _cookie: &str) {}
    fn has_unix_socket(&self) -> bool {
        false
    }
    fn set_unix_socket(&mut self, _socket: &str) {}
    fn match_wildcard(&mut self, _opt: bool) {}
    fn set_method(&mut self, _method: &str) {}
    fn write_output(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn set_auth(&mut self, _auth: super::curl::AuthKind) {}
    fn add_headers(&mut self, _headers: &str) {}
    fn enable_response_headers(&mut self, _opt: bool) {}
    fn enable_progress_bar(&mut self, _opt: bool) {}
    fn set_referrer(&mut self, _referrer: &str) {}
    fn set_user_agent(&mut self, _agent: &str) {}
    fn set_max_redirects(&mut self, _redirects: usize) {}
    fn set_ca_path(&mut self, _path: &str) {}
    fn set_tcp_keepalive(&mut self, _opt: bool) {}
    fn set_fail_on_error(&mut self, _opt: bool) {}
    fn set_cert_info(&mut self, _opt: bool) {}
    fn save_command(&mut self, _opt: bool) {}
    fn set_verbose(&mut self, _opt: bool) {}
    fn save_token(&mut self, _opt: bool) {}
    fn get_token(&self) -> Option<String> {
        None
    }
    fn remove_headers(&mut self, _headers: &str) {}
    fn will_save_command(&self) -> bool {
        false
    }
}

impl Default for Wget {
    fn default() -> Self {
        Self::new()
    }
}
impl Wget {
    pub fn new() -> Self {
        Wget {
            cmd: vec![String::from("wget")],
            rec_level: None,
            url: String::new(),
            auth: None,
            output: String::new(),
            response: None,
        }
    }

    pub fn has_url(&self) -> bool {
        !self.url.is_empty()
    }

    fn has_output(&self) -> bool {
        !self.output.is_empty()
    }

    fn has_rec(&self) -> bool {
        self.rec_level.is_some()
    }

    // This just builds a vector of strings for executing in command.arg(str)

    pub fn build_command(&mut self) {
        self.cmd.push(self.url.clone());
        if self.has_rec() {
            self.cmd.push("--depth".to_string());
            if self.has_rec() {
                self.cmd.push(self.rec_level.unwrap().to_string());
            }
        }
        if let Some(ref auth) = self.auth {
            self.cmd.push(auth.clone());
        }
        if self.has_output() {
            self.cmd.push(format!("-O {}", self.output));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::ServerGuard;

    fn setup() -> ServerGuard {
        let mut server = mockito::Server::new();
        // Start a mock server
        let _ = server
            .mock("GET", "/api/resource")
            .with_status(200)
            .with_body("Mocked Response")
            .create();
        server
    }

    #[test]
    fn test_new_wget() {
        let wget = Wget::new();
        assert_eq!("wget", wget.cmd.get(0).unwrap());
        assert_eq!("", wget.url);
        assert_eq!("", wget.output);
    }

    #[test]
    fn test_set_url() {
        let mut wget = Wget::new();
        wget.set_url("https://www.google.com");
        wget.build_command_string();
        assert_eq!("wget https://www.google.com", wget.get_command_string());
    }

    #[test]
    fn test_set_output() {
        let mut wget = Wget::new();
        wget.set_outfile("output.txt");
        wget.build_command_string();
        assert_eq!("wget -O output.txt", wget.get_command_string());
    }

    #[test]
    fn test_add_auth() {
        let mut wget = Wget::new();
        wget.add_basic_auth("usr:pwd");
        wget.build_command_string();
        assert_eq!("wget --user=usr --password=pwd", wget.get_command_string());
    }

    #[test]
    fn test_build_string() {
        let mut wget = Wget::new();
        wget.add_basic_auth("usr:pwd");
        wget.set_url("http://www.google.com");
        wget.set_outfile("output");
        wget.build_command_string();
        assert_eq!(
            "wget http://www.google.com --user=usr --password=pwd -O output",
            wget.get_command_string()
        );
    }

    #[test]
    fn test_increase_rec_level() {
        let mut wget = Wget::new();
        wget.set_rec_download_level(2);
        wget.build_command_string();
        assert_eq!("wget -r 2", wget.get_command_string());
        assert_eq!(2, wget.rec_level.unwrap());
    }

    #[test]
    fn test_execute() {
        use std::ops::DerefMut;

        let mut setup = setup();
        let url = setup.deref_mut().url().clone();
        let mut wget = Wget::new();
        wget.set_url(url.clone().as_str());
        wget.set_outfile("output");
        wget.build_command_string();
        assert_eq!(format!("wget {} -O output", url), wget.get_command_string());
        if wget.execute(None).is_ok() {
            assert!(std::fs::metadata("output").is_ok());
            std::fs::remove_file("output").unwrap();
        }
    }
}
