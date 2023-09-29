#[derive(Debug, Clone, PartialEq)]
pub struct Wget {
    cmd: String,
    rec_level: usize,
    auth: Option<String>,
    url: String,
    output: String,
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
        }
    }

    #[cfg(target_os = "windows")]
    pub fn new() -> Self {
        Wget {
            cmd: String::new(),
            rec_level: 0,
            url: String::new(),
            auth: None,
            output: String::new(),
        }
    }

    // The menu must add the recursion level, then the url, then the output
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

    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub fn add_auth(&mut self, usr: &str, pwd: &str) {
        self.auth = Some(format!("--user={} --password={}", usr, pwd));
    }

    #[cfg(target_os = "windows")]
    pub fn add_auth(&mut self, usr: &str, pwd: &str) {
        self.auth = Some(format!("-username {} -password {}", usr, pwd));
    }

    pub fn set_output(&mut self, output: String) {
        self.output = output;
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

    #[cfg(any(target_os = "windows"))]
    pub fn build_string(&mut self) {
        // URL
        if self.has_url() {
            self.cmd.push_str("-url");
            self.cmd.push(' '); // Whitespace
            self.cmd.push_str(self.url.as_str());
            self.cmd.push(' '); // Whitespace
        }

        // REC
        if self.has_rec() {
            self.cmd.push_str("-depth");
            self.cmd.push(' '); // Whitespace
            self.cmd.push_str(format!("{}", self.rec_level).as_str());
            self.cmd.push(' '); // Whitespace
        }

        // AUTH
        if self.has_auth() {
            self.cmd.push_str(self.auth.as_ref().unwrap().as_str());
            self.cmd.push(' '); // Whitespace
        }

        // OUTPUT
        if self.has_output() {
            self.cmd.push_str("-outputfile");
            self.cmd.push(' '); // Whitespace
            self.cmd.push_str(self.output.as_str());
            self.cmd.push(' '); // Whitespace
        }

        self.cmd = self.cmd.trim().to_string();
    }

    // Added Windows Specific Function
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub fn execute(&mut self) -> Result<String, String> {
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(self.cmd.clone())
            .output()
            .expect("failed to execute process");
        if output.status.success() {
            Ok(String::from_utf8(output.stdout).unwrap())
        } else {
            Err(String::from_utf8(output.stderr).unwrap())
        }
    }

    #[cfg(target_os = "windows")]
    pub fn execute(&mut self) -> Result<String, String> {
        let output = std::process::Command::new("powershell.exe")
            .arg("-NoLogo")
            .arg("-NoProfile")
            .arg("-ExecutionPolicy")
            .arg("Unrestricted")
            .arg("-File")
            .arg("scripts\\win64-wget.ps1")
            .arg(self.cmd.clone())
            .output()
            .expect("failed to execute process");
        let outstr = format!("{:?}", output);
        println!("{}", outstr);
        if output.status.success() {
            Ok(String::from_utf8(output.stdout).unwrap())
        } else {
            Err(String::from_utf8(output.stderr).unwrap())
        }
    }
}

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
    assert_eq!("", wget.cmd);
    assert_eq!("", wget.url);
    assert_eq!("", wget.output);
}

#[test]
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn test_set_url() {
    let mut wget = Wget::new();
    wget.set_url(String::from("http://www.google.com"));
    wget.build_string();
    assert_eq!("wget http://www.google.com", wget.cmd);
}

#[test]
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn test_set_output() {
    let mut wget = Wget::new();
    wget.set_output(String::from("output"));
    wget.build_string();
    assert_eq!("wget -O output", wget.cmd);
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
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn test_build_string() {
    let mut wget = Wget::new();
    wget.add_auth("usr", "pwd");
    wget.set_url(String::from("http://www.google.com"));
    wget.set_output(String::from("output"));
    wget.build_string();
    assert_eq!(
        "wget http://www.google.com --user=usr --password=pwd -O output",
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
    wget.set_url(String::from("http://www.google.com"));
    wget.set_output(String::from("output"));
    wget.build_string();
    assert_eq!("wget http://www.google.com -O output", wget.cmd);
    let result = wget.execute();
    assert_eq!(true, result.is_ok());
    assert!(std::fs::metadata("output").is_ok());
    std::fs::remove_file("output").unwrap();
}

#[test]
#[cfg(target_os = "windows")]
fn test_execute() {
    let mut wget = Wget::new();
    wget.set_url(String::from("http://www.google.com"));
    wget.set_output(String::from("output"));
    wget.build_string();
    assert_eq!("-url http://www.google.com -outputfile output", wget.cmd);
    let result = wget.execute();
    assert_eq!(true, result.is_ok());
    assert!(std::fs::metadata("output.txt").is_ok());
    std::fs::remove_file("output.txt").unwrap();
}
