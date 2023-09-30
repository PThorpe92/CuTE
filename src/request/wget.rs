#[derive(Debug, Clone, PartialEq)]
pub struct Wget {
    cmd: String,
    rec_level: usize,
    auth: Option<String>,
    url: String,
    output: String,
    response: Option<String>,
}

impl Wget {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub fn new() -> Self {
        Wget {
            cmd: String::from("wget"),
            rec_level: 0,
            url: String::new(),
            auth: None,
            output: String::new(),
            response: None,
        }
    }

    #[cfg(target_os = "windows")]
    pub fn new() -> Self {
        Wget {
            cmd: String::from("powershell.exe -NoLogo -NoProfile -ExecutionPolicy Unrestricted -File scripts\\win64-wget.ps1"),
            rec_level: 0,
            url: String::new(),
            auth: None,
            output: String::new(),
            response: None,
        }
    }

    #[cfg(target_os = "windows")]
    pub fn build_string(&mut self) {
        if self.has_url() {
            if !self.cmd.as_bytes()[self.cmd.len() - 1].is_ascii_whitespace() {
                self.cmd.push(' ');
            }
            self.cmd.push_str("-url");
            self.cmd.push(' ');
            self.cmd.push_str(self.url.as_str());
        }
        if self.has_rec() {
            if !self.cmd.as_bytes()[self.cmd.len() - 1].is_ascii_whitespace() {
                self.cmd.push(' ');
            }
            self.cmd
                .push_str(format!("-depth {}", self.rec_level).as_str());
        }
        if self.has_auth() {
            if !self.cmd.as_bytes()[self.cmd.len() - 1].is_ascii_whitespace() {
                self.cmd.push(' ');
            }
            self.cmd.push_str(self.auth.as_ref().unwrap().as_str());
        }
        if self.has_output() {
            if !self.cmd.as_bytes()[self.cmd.len() - 1].is_ascii_whitespace() {
                self.cmd.push(' ');
            }
            self.cmd
                .push_str(format!("-outputfile {}", self.output).as_str());
        }
        self.cmd = self.cmd.trim().to_string();
    }

    pub fn get_response(&self) -> Option<String> {
        self.response.clone()
    }

    pub fn set_rec_download_level(&mut self, level: usize) {
        self.rec_level = level;
    }

    pub fn has_url(&self) -> bool {
        !self.url.is_empty()
    }

    pub fn has_output(&self) -> bool {
        !self.output.is_empty()
    }

    pub fn has_auth(&self) -> bool {
        self.auth.is_some()
    }

    pub fn has_rec(&self) -> bool {
        self.cmd.contains("-r")
    }

    pub fn set_url(&mut self, url: &str) {
        self.url = String::from(url);
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub fn add_auth(&mut self, usr: &str, pwd: &str) {
        self.auth = Some(format!("--user={} --password={}", usr, pwd));
    }

    #[cfg(target_os = "windows")]
    pub fn add_auth(&mut self, usr: &str, pwd: &str) {
        self.auth = Some(format!("-username {} -password {}", usr, pwd));
    }

    pub fn set_output(&mut self, output: &str) {
        self.output = String::from(output);
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub fn build_string(&mut self) {
        if self.has_url() {
            if !self.cmd.as_bytes()[self.cmd.len() - 1].is_ascii_whitespace() {
                self.cmd.push(' ');
            }
            self.cmd.push_str(self.url.as_str());
        }
        if self.has_rec() {
            if !self.cmd.as_bytes()[self.cmd.len() - 1].is_ascii_whitespace() {
                self.cmd.push(' ');
            }
            self.cmd.push_str(format!("-r {}", self.rec_level).as_str());
        }
        if self.has_auth() {
            if !self.cmd.as_bytes()[self.cmd.len() - 1].is_ascii_whitespace() {
                self.cmd.push(' ');
            }
            self.cmd.push_str(self.auth.as_ref().unwrap().as_str());
        }
        if self.has_output() {
            if !self.cmd.as_bytes()[self.cmd.len() - 1].is_ascii_whitespace() {
                self.cmd.push(' ');
            }
            self.cmd.push_str(format!("-O {}", self.output).as_str());
        }
        self.cmd = self.cmd.trim().to_string();
    }

    // Added Windows Specific Function
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub fn execute(&mut self) -> Result<(), String> {
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(self.cmd.clone())
            .output()
            .expect("failed to execute process");
        if output.status.success() {
            self.response = Some(String::from_utf8(output.stdout).unwrap());
            Ok(())
        } else {
            Err(String::from_utf8(output.stderr).unwrap())
        }
    }
}

    #[cfg(target_os = "windows")]
    pub fn execute(&mut self) -> Result<(), String> {
        let commands = self.cmd.split(" ").collect::<Vec<&str>>();
        let args = commands[7..].join(" ");

        let output = std::process::Command::new(commands[0])
            .arg(commands[1]) // -NoLogo
            .arg(commands[2]) // -NoProfile
            .arg(commands[3]) // -ExecutionPolicy
            .arg(commands[4]) // unrestricted
            .arg(commands[5]) // -File
            .arg(commands[6]) // scripts\\win64-wget.ps1
            .arg(args) // Rest Of The Command Arguments
            .output()
            .expect("failed to execute process");
        if output.status.success() {
            self.response = Some(String::from_utf8(output.stdout).unwrap());
            Ok(())
        } else {
            Err(String::from_utf8(output.stderr).unwrap())
        }
    }


mod tests {
    use crate::request::wget::Wget;

    #[test]
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn test_new_wget() {
        let wget = Wget::new();
        assert_eq!("wget", wget.cmd);
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
        wget.set_url("http://www.google.com");
        wget.build_string();
        assert_eq!("wget http://www.google.com", wget.cmd);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_set_url_win() {
        let mut wget = Wget::new();
        wget.set_url("http://www.google.com");
        wget.build_string();
        assert_eq!("powershell.exe -NoLogo -NoProfile -ExecutionPolicy Unrestricted -File scripts\\win64-wget.ps1 -url http://www.google.com", wget.cmd);
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn test_set_output() {
        let mut wget = Wget::new();
        wget.set_output("output");
        wget.build_string();
        assert_eq!("wget -O output", wget.cmd);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_set_output() {
        let mut wget = Wget::new();
        wget.set_output("output");
        wget.build_string();
        assert_eq!("powershell.exe -NoLogo -NoProfile -ExecutionPolicy Unrestricted -File scripts\\win64-wget.ps1 -outputfile output", wget.cmd);
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn test_add_auth() {
        let mut wget = Wget::new();
        wget.add_auth("usr", "pwd");
        wget.build_string();
        assert_eq!("wget --user=usr --password=pwd", wget.cmd);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_add_auth() {
        let mut wget = Wget::new();
        wget.add_auth("usr", "pwd");
        wget.build_string();
        assert_eq!("powershell.exe -NoLogo -NoProfile -ExecutionPolicy Unrestricted -File scripts\\win64-wget.ps1 -username usr -password pwd", wget.cmd);
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn test_build_string() {
        let mut wget = Wget::new();
        wget.add_auth("usr", "pwd");
        wget.set_url("http://www.google.com");
        wget.set_output("output");
        wget.build_string();
        assert_eq!(
            "wget http://www.google.com --user=usr --password=pwd -O output",
            wget.cmd
        );
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_build_string() {
        let mut wget = Wget::new();
        wget.add_auth("usr", "pwd");
        wget.set_url("http://www.google.com");
        wget.set_output("output");
        wget.build_string();
        assert_eq!(
            "powershell.exe -NoLogo -NoProfile -ExecutionPolicy Unrestricted -File scripts\\win64-wget.ps1 -url http://www.google.com -username usr -password pwd -outputfile output",
            wget.cmd
        );
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn test_increase_rec_level() {
        let mut wget = Wget::new();
        wget.set_rec_download_level(2);
        assert_eq!("wget", wget.cmd);
        assert_eq!(2, wget.rec_level);
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn test_execute() {
        let mut wget = Wget::new();
        wget.set_url("http://www.google.com");
        wget.set_output("output");
        wget.build_string();
        assert_eq!("wget http://www.google.com -O output", wget.cmd);
        let result = wget.execute();
        assert_eq!(true, result.is_ok());
        assert!(std::fs::metadata("output").is_ok());
        std::fs::remove_file("output").unwrap();
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_increase_rec_level() {
        let mut wget = Wget::new();
        wget.set_rec_download_level(2);
        assert_eq!("powershell.exe -NoLogo -NoProfile -ExecutionPolicy Unrestricted -File scripts\\win64-wget.ps1", wget.cmd);
        assert_eq!(2, wget.rec_level);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_execute() {
        let mut wget = Wget::new();
        wget.set_url("http://www.google.com");
        wget.set_output("output");
        wget.build_string();
        assert_eq!("powershell.exe -NoLogo -NoProfile -ExecutionPolicy Unrestricted -File scripts\\win64-wget.ps1 -url http://www.google.com -outputfile output", wget.cmd);
        let result = wget.execute();
        assert_eq!(true, result.is_ok());
        assert!(std::fs::metadata("output.txt").is_ok());
        std::fs::remove_file("output.txt").unwrap();
    }
}
