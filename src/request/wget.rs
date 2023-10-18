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
    #[cfg(target_os = "windows")]
    fn execute(&mut self, db: Option<&mut Box<DB>>) -> Result<(), String> {
        self.build_command();
        let output = std::process::Command::new(commands[0])
            .arg(self.cmd.as_slice()) // Rest Of The Command Arguments
            .output()
            .expect("failed to execute process");
        if output.status.success() {
            self.response = Some(String::from_utf8(output.stdout).unwrap());
            Ok(())
        } else {
            Err(String::from_utf8(output.stderr).unwrap())
        }
    }

    // This is just if we want to build a string from the command. Not sure if we are going
    // to need this, but we may end up storing download commands in DB
    #[cfg(any(target_os = "linux", target_os = "macos"))]
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

    fn set_response(&mut self, response: &str) {
        self.response = Some(response.to_string());
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
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
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn add_basic_auth(&mut self, info: &str) {
        let mut split = info.split(':');
        let usr = split.next().unwrap();
        let pwd = split.next().unwrap();
        self.auth = Some(format!("--user={} --password={}", usr, pwd));
    }

    #[cfg(target_os = "windows")]
    fn build_command_string(&self) -> String {
        let mut cmdstr = vec![String::from("powershell.exe -NoLogo -NoProfile -ExecutionPolicy Unrestricted -File scripts\\win64-wget.ps1")];
        if self.has_url() {
            cmdstr.push("-url".to_string());
            cmdstr.push(self.url);
        }
        if self.has_rec() {
            cmdstr.push(format!("-depth {}", self.rec_level).as_str());
        }
        if self.has_auth() {
            cmdstr.push(self.auth.as_ref().unwrap());
        }
        if self.has_output() {
            cmdstr.push(format!("-outputfile {}", self.output));
        }
        return cmdstr.join(" ").trim().to_string();
    }

    #[cfg(target_os = "windows")]
    fn add_auth(&mut self, usr: &str, pwd: &str) {
        self.auth = Some(format!("-username {} -password {}", usr, pwd));
    }

    fn set_rec_download_level(&mut self, level: usize) {
        self.rec_level = Some(level);
    }

    fn set_url(&mut self, url: &str) {
        self.url = url.to_string();
    }

    fn get_url(&self) -> String {
        self.url.clone()
    }

    fn has_auth(&self) -> bool {
        self.auth.is_some()
    }

    fn set_outfile(&mut self, file: &str) {
        self.output = file.to_string();
    }
    fn get_response(&self) -> String {
        self.response
            .clone()
            .unwrap_or(String::from("No download response"))
    }
}

impl Default for Wget {
    fn default() -> Self {
        Self::new()
    }
}
impl Wget {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
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

    #[cfg(target_os = "windows")]
    pub fn new() -> Self {
        Wget {
            cmd: vec![
                "powershell.exe".to_string(),
                "-NoLogo".to_string(),
                "-NoProfile".to_string(),
                "-ExecutionPolicy".to_string(),
                "Unrestricted".to_string(),
                "-File".to_string(),
                "scripts\\win64-wget.ps1".to_string(),
            ],
            rec_level: 0,
            url: String::new(),
            auth: None,
            output: String::new(),
            response: None,
        }
    }

    #[cfg(target_os = "windows")]
    pub fn build_command(&mut self) {
        if self.has_url() {
            self.cmd.push("-url".to_string());
        }
        if self.has_rec() {
            self.cmd.push(format!("-depth {}", self.rec_level).as_str());
        }
        if self.has_auth() {
            self.cmd.push(self.auth.as_ref().unwrap());
        }
        if self.has_output() {
            self.cmd.push(format!("-outputfile {}", self.output));
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
    #[cfg(any(target_os = "linux", target_os = "macos"))]
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
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn test_new_wget() {
        let wget = Wget::new();
        assert_eq!("wget", wget.cmd.get(0).unwrap());
        assert_eq!("", wget.url);
        assert_eq!("", wget.output);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_new_wget() {
        let wget = Wget::new();
        assert_eq!("powershell.exe -NoLogo -NoProfile -ExecutionPolicy Unrestricted -File scripts\\win64-wget.ps1", wget.cmd);
        assert_eq!("", wget.url);
        assert_eq!("", wget.output);
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn test_set_url() {
        let mut wget = Wget::new();
        wget.set_url("https://www.google.com");
        assert_eq!("wget https://www.google.com", wget.get_command_string());
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_set_url_win() {
        let mut wget = Wget::new();
        wget.set_url("http://www.google.com");
        assert_eq!("powershell.exe -NoLogo -NoProfile -ExecutionPolicy Unrestricted -File scripts\\win64-wget.ps1 -url http://www.google.com", wget.get_command_string());
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn test_set_output() {
        let mut wget = Wget::new();
        wget.set_outfile("output.txt");
        assert_eq!("wget -O output.txt", wget.get_command_string());
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_set_output() {
        let mut wget = Wget::new();
        wget.set_output("output");
        assert_eq!("powershell.exe -NoLogo -NoProfile -ExecutionPolicy Unrestricted -File scripts\\win64-wget.ps1 -outputfile output", wget.get_command_string());
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn test_add_auth() {
        let mut wget = Wget::new();
        wget.add_basic_auth("usr:pwd");
        assert_eq!("wget --user=usr --password=pwd", wget.get_command_string());
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_add_auth() {
        let mut wget = Wget::new();
        wget.add_basic_auth("usr:pwd");
        assert_eq!("powershell.exe -NoLogo -NoProfile -ExecutionPolicy Unrestricted -File scripts\\win64-wget.ps1 -username usr -password pwd", wget.get_command_string());
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "macos"))]
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
    #[cfg(target_os = "windows")]
    fn test_build_string() {
        let mut wget = Wget::new();
        wget.add_basic_auth("usr:pwd");
        wget.set_url("http://www.google.com");
        wget.set_output("output");
        assert_eq!(
            "powershell.exe -NoLogo -NoProfile -ExecutionPolicy Unrestricted -File scripts\\win64-wget.ps1 -url http://www.google.com -username usr -password pwd -outputfile output",
            wget.get_command_string()
        );
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn test_increase_rec_level() {
        let mut wget = Wget::new();
        wget.set_rec_download_level(2);
        assert_eq!("wget -r 2", wget.get_command_string());
        assert_eq!(2, wget.rec_level.unwrap());
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "macos"))]
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

    #[test]
    #[cfg(target_os = "windows")]
    fn test_increase_rec_level() {
        let mut wget = Wget::new();
        wget.set_rec_download_level(2);
        assert_eq!("powershell.exe -NoLogo -NoProfile -ExecutionPolicy Unrestricted -File scripts\\win64-wget.ps1", wget.get_command_string());
        assert_eq!(2, wget.rec_level.unwrap());
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_execute() {
        let mut wget = Wget::new();
        wget.set_url("http://www.google.com");
        wget.set_outfile("output");
        assert_eq!("powershell.exe -NoLogo -NoProfile -ExecutionPolicy Unrestricted -File scripts\\win64-wget.ps1 -url http://www.google.com -outputfile output", wget.get_command_string());
        let result = wget.execute();
        assert_eq!(true, result.is_ok());
        assert!(std::fs::metadata("output.txt").is_ok());
        std::fs::remove_file("output.txt").unwrap();
    }
}
