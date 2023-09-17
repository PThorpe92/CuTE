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
    pub fn set_download(&mut self, download: bool) {
        self.download = download;
    }

    pub fn set_method(&mut self, method: String) {
        let mut new_method = String::from(" --method=");
        new_method.push_str(method.as_str());
        self.cmd.push_str(new_method.as_str());
    }
}
