use std::io::Write;

#[derive(Debug, Clone, PartialEq)]
pub struct Wget {
    pub cmd: String,
    pub url: String,
    pub rec_download: u8,
    pub output: String,
    pub response: String,
}

impl Wget {
    pub fn new() -> Self {
        Wget {
            cmd: String::from("wget"),
            url: String::new(),
            rec_download: 0,
            output: String::new(),
            response: String::new(),
        }
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn set_output(&mut self, output: String) {
        self.output = output;
    }
    pub fn set_response(&mut self, response: String) {
        self.response = response.clone();
    }
    pub fn write_output(&mut self) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::create(self.output.clone())?;
        let mut writer = std::io::BufWriter::new(&mut file);
        writer
            .write_all(self.response.clone().as_bytes())
            .expect("failed to write file");
        Ok(())
    }

    pub fn execute(&mut self) -> Result<String, String> {
        let mut cmd = String::from(self.cmd.as_str());
        cmd.push(' ');
        cmd.push_str(self.url.as_str());
        cmd.push_str(" -O ");
        cmd.push_str(self.output.as_str());
        if self.rec_download > 0 {
            let level = format!("-r --level={}", self.rec_download);
            cmd.push_str(level.as_str());
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
        } else if self.cmd.contains("--verbose") {
            self.cmd = self.cmd.replace("--verbose", "");
        }
    }

    pub fn is_recursive(&self) -> bool {
        self.cmd.contains("-r")
    }

    pub fn set_recursive_download(&mut self, download: u8) {
        self.rec_download = download;
    }

    pub fn set_method(&mut self, method: String) {
        let mut new_method = String::from(" --method=");
        new_method.push_str(method.as_str());
        self.cmd.push_str(new_method.as_str());
    }
}
