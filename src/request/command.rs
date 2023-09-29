use crate::request::curl::{Curl, CurlFlag};
use crate::request::wget::Wget;

#[derive(Debug)]
pub enum Command<'a> {
    Curl(Curl<'a>),
    Wget(Wget),
}

impl<'a> Command<'a> {
    pub fn default(curl: Curl<'a>) -> Self {
        Command::Curl(curl)
    }

    pub fn set_method(&mut self, method: String) {
        match method.as_str() {
            "GET" => match self {
                Command::Curl(curl) => curl.set_get_method(true),
                _ => {}
            },
            "POST" => match self {
                Command::Curl(curl) => curl.set_post_method(true),
                _ => {}
            },
            "PUT" => match self {
                Command::Curl(curl) => curl.set_put_method(true),
                _ => {}
            },
            "PATCH" => match self {
                Command::Curl(curl) => curl.set_patch_method(true),
                _ => {}
            },
            "DELETE" => match self {
                Command::Curl(curl) => curl.set_delete_method(true),
                _ => {}
            },
            _ => {}
        }
    }

    pub fn add_download_auth(&mut self, user: &str, pwd: &str) {
        match self {
            Command::Wget(wget) => wget.add_auth(user, pwd),
            _ => {}
        }
    }

    pub fn set_url(&mut self, url: String) {
        match self {
            Command::Curl(curl) => {
                curl.set_url(url.clone());
            }
            Command::Wget(wget) => {
                wget.set_url(url);
            }
        }
    }

    pub fn set_outfile(&mut self, file: String) {
        match self {
            Command::Curl(curl) => {
                curl.add_flag(CurlFlag::Output(""), Some(file));
            }
            Command::Wget(wget) => {
                wget.set_output(file);
            }
        }
    }

    pub fn set_headers(&mut self, headers: Vec<String>) {
        match self {
            Command::Curl(curl) => {
                curl.add_headers(headers);
            }
            _ => {}
        }
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        match self {
            Command::Curl(curl) => curl.set_verbose(verbose),
            _ => {}
        }
    }

    pub fn execute(&mut self) -> Result<String, std::io::Error> {
        match self {
            Command::Curl(curl) => Ok(curl.execute().unwrap_or("".to_string())),
            Command::Wget(wget) => Ok(wget.execute().unwrap_or("".to_string())),
        }
    }

    pub fn set_rec_download_level(&mut self, level: usize) {
        match self {
            Command::Wget(wget) => wget.set_rec_download_level(level),
            _ => {}
        }
    }

    pub fn write_output(&mut self) -> Result<(), std::io::Error> {
        match self {
            Command::Curl(curl) => {
                curl.write_output()?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn set_response(&mut self, response: String) {
        match self {
            Command::Curl(curl) => {
                curl.set_response(response.clone());
            }
            _ => {}
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Command::Curl(_) => "Curl",
            Command::Wget(_) => "Wget",
        }
        .to_string()
    }
}
