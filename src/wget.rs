#[derive(Debug, Clone, PartialEq)]
pub struct Wget<'a> {
    pub cmd: &'a str,
    pub url: &'a str,
    pub download: bool,
    pub output: &'a str,
}
impl<'a> Wget<'a> {
    pub fn new(method: &'a str) -> Self {
        Wget {
            cmd: method,
            url: "",
            download: false,
            output: "",
        }
    }
}
