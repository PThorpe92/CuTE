use crate::request::curl::{Curl, CurlFlag};
use crate::request::request::Request;
use crate::request::wget::Wget;

#[derive(Debug)]
pub enum Command<'a> {
    Curl(Curl<'a>),
    Wget(Wget),
    Custom(Request),
}

impl<'a> Command<'a> {
    pub fn default(curl: Curl<'a>) -> Self {
        Command::Curl(curl)
    }

    pub fn set_method(&mut self, method: String) {
        match method.as_str() {
            "GET" => match self {
                Command::Curl(curl) => curl.set_get_method(true),
                Command::Wget(wget) => wget.set_method(method),
                Command::Custom(req) => req.method = method,
            },
            "POST" => match self {
                Command::Curl(curl) => curl.set_post_method(true),
                Command::Wget(wget) => wget.set_method(method),
                Command::Custom(req) => req.method = method,
            },
            "PUT" => match self {
                Command::Curl(curl) => curl.set_put_method(true),
                Command::Wget(wget) => wget.set_method(method),
                Command::Custom(req) => req.method = method,
            },
            "PATCH" => match self {
                Command::Curl(curl) => curl.set_patch_method(true),
                Command::Wget(wget) => wget.set_method(method),
                Command::Custom(req) => req.method = method,
            },
            "DELETE" => match self {
                Command::Curl(curl) => curl.set_delete_method(true),
                Command::Wget(wget) => wget.set_method(method),
                Command::Custom(req) => req.method = method,
            },
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
            Command::Custom(req) => {
                req.url = url;
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
            Command::Custom(req) => {
                req.output = Some(file.clone());
            }
        }
    }

    pub fn add_headers(&mut self, headers: (String, String)) {
        match self {
            Command::Custom(req) => {
                req.add_headers(headers);
            }
            _ => {}
        }
    }
    pub fn set_headers(&mut self, headers: Vec<String>) {
        match self {
            Command::Curl(curl) => {
                curl.add_headers(headers);
            }
            Command::Custom(_) => {
                //req.set_headers(headers);
            }
            _ => {}
        }
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        match self {
            Command::Curl(curl) => curl.set_verbose(verbose),
            Command::Wget(wget) => {
                wget.set_verbose(verbose);
            }
            Command::Custom(_) => {}
        }
    }

    pub fn set_rec_download(&mut self, level: usize) {
        match self {
            Command::Wget(wget) => {
                wget.set_recursive_download(level as u8);
            }
            _ => {}
        }
    }

    pub async fn execute(&mut self) -> Result<String, std::io::Error> {
        match self {
            Command::Curl(curl) => Ok(curl.execute().unwrap_or("".to_string())),
            Command::Wget(wget) => Ok(wget.execute().unwrap_or("".to_string())),
            Command::Custom(req) => Ok(req.send_request().await.unwrap_or_default()),
        }
    }
    pub fn write_output(&mut self) -> Result<(), std::io::Error> {
        match self {
            Command::Curl(curl) => {
                curl.write_output()?;
                Ok(())
            }
            Command::Wget(wget) => {
                wget.write_output()?;
                Ok(())
            }
            Command::Custom(_) => Ok(()),
        }
    }

    pub fn set_response(&mut self, response: String) {
        match self {
            Command::Curl(curl) => {
                curl.set_response(response.clone());
            }
            Command::Wget(wget) => {
                wget.set_response(response.clone());
            }
            Command::Custom(req) => {
                req.set_response(response.clone());
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Command::Curl(_) => "Curl",
            Command::Wget(_) => "Wget",
            Command::Custom(_) => "Custom",
        }
        .to_string()
    }
}
