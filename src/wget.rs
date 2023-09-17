#[derive(Debug, Clone, PartialEq)]
pub struct Wget<'a> {
    pub cmd: String,
    pub url: &'a str,
    pub download: bool,
    pub output: &'a str,
}
impl<'a> Wget<'a> {
    pub fn new() -> Self {
        Wget {
            cmd: String::from("wget"),
            url: "",
            download: false,
            output: "",
        }
    }
    pub fn set_url(&mut self, url: &'a str) {
        self.url = url;
    }
    pub fn set_output(&mut self, output: &'a str) {
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
