#[derive(Debug, Clone, PartialEq)]
pub struct Wget {
    pub cmd: String,
    pub url: String,
    pub download: bool,
    pub output: String,
}
impl Wget {
    pub fn new() -> Self {
        Wget {
            cmd: String::from("wget"),
            url: String::new(),
            download: false,
            output: String::new(),
        }
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn set_output(&mut self, output: String) {
        self.output = output;
    }

    pub fn execute(&mut self) -> Result<String, String> {
        let mut cmd = String::from(self.cmd.as_str());
        cmd.push_str(" ");
        cmd.push_str(self.url.as_str());
        cmd.push_str(" -O ");
        cmd.push_str(self.output.as_str());
        if self.download {
            cmd.push_str(" --continue");
        }
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("failed to execute process");
        if output.status.success() {
            Ok(String::from_utf8(output.stdout).unwrap())
        } else {
            Err(String::from_utf8(output.stderr).unwrap())
        }
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        if verbose {
            self.cmd.push_str(" --verbose");
        } else {
            if self.cmd.contains("--verbose") {
                self.cmd = self.cmd.replace("--verbose", "");
            }
        }
    }

    pub fn set_download(&mut self, download: bool) {
        self.download = download;
    }

    pub fn set_method(&mut self, method: String) {
        let mut new_method = String::from(" --method=");
        new_method.push_str(method.as_str());
        self.cmd.push_str(new_method.as_str());
    }
}
