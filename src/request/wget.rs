#[derive(Debug, Clone, PartialEq)]
pub struct Wget {
    cmd: String,
    rec_level: usize,
    auth: Option<String>,
    url: String,
    output: String,
}

impl Wget {
    pub fn new() -> Self {
        Wget {
            cmd: String::from("wget"),
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

    pub fn add_auth(&mut self, usr: &str, pwd: &str) {
        self.auth = Some(format!("--user={} --password={}", usr, pwd));
    }

    pub fn set_output(&mut self, output: String) {
        self.output = output;
    }

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
}

#[test]
fn test_new_wget() {
    let wget = Wget::new();
    assert_eq!("wget", wget.cmd);
    assert_eq!("", wget.url);
    assert_eq!("", wget.output);
}

#[test]
fn test_set_url() {
    let mut wget = Wget::new();
    wget.set_url(String::from("http://www.google.com"));
    wget.build_string();
    assert_eq!("wget http://www.google.com", wget.cmd);
}

#[test]
fn test_set_output() {
    let mut wget = Wget::new();
    wget.set_output(String::from("output"));
    wget.build_string();
    assert_eq!("wget -O output", wget.cmd);
}

#[test]
fn test_add_auth() {
    let mut wget = Wget::new();
    wget.add_auth("usr", "pwd");
    wget.build_string();
    assert_eq!("wget --user=usr --password=pwd", wget.cmd);
}
#[test]
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
fn test_increase_rec_level() {
    let mut wget = Wget::new();
    wget.set_rec_download_level(2);
    assert_eq!("wget", wget.cmd);
    assert_eq!(2, wget.rec_level);
}

#[test]
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
