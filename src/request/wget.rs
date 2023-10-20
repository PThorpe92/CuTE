use std::fmt::format;
use crate::database::db::DB;

use super::command::CmdOpts;

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct Wget {
    cmd: Vec<String>,
    rec_level: Option<usize>,
    auth: Option<String>,
    url: String,
    output: String,
    response: Option<String>,
}

impl CmdOpts for Wget {

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


    fn get_url(&self) -> String {
        self.url.clone()
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

    fn get_command_string(&mut self) -> String {
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
        return cmdstr.join(" ").trim().to_string();
    }
    fn has_auth(&self) -> bool {
        self.auth.is_some()
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
        assert_eq!("wget https://www.google.com", wget.get_command_string());
    }


    #[test]
    fn test_set_output() {
        let mut wget = Wget::new();
        wget.set_outfile("output.txt");
        assert_eq!("wget -O output.txt", wget.get_command_string());
    }


    #[test]
    fn test_add_auth() {
        let mut wget = Wget::new();
        wget.add_basic_auth("usr:pwd");
        assert_eq!("wget --user=usr --password=pwd", wget.get_command_string());
    }


    #[test]
    fn test_build_string() {
        let mut wget = Wget::new();
        wget.add_basic_auth("usr:pwd");
        wget.set_url("http://www.google.com");
        wget.set_outfile("output");
        assert_eq!(
            "wget http://www.google.com --user=usr --password=pwd -O output",
            wget.get_command_string()
        );
    }



    #[test]
    fn test_increase_rec_level() {
        let mut wget = Wget::new();
        wget.set_rec_download_level(2);
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
        assert_eq!(format!("wget {} -O output", url), wget.get_command_string());
        if wget.execute(None).is_ok() {
            assert!(std::fs::metadata("output").is_ok());
            std::fs::remove_file("output").unwrap();
        }
    }

}
