use crate::request::curl::{Curl, CurlFlag};
use crate::request::wget::Wget;

use super::curl::{AuthKind, CurlFlagType};

#[derive(Debug)]
pub enum Command<'a> {
    Curl(Curl<'a>),
    Wget(Wget),
}

impl<'a> Command<'a> {
    // Im gonna fix this. We should be implmenting the default trait for Command

    pub fn set_method(&mut self, method: String) {
        match method.as_str() {
            "GET" => match self {
                Command::Curl(curl) => curl.set_get_method(),
                _ => {}
            },
            "POST" => match self {
                Command::Curl(curl) => curl.set_post_method(),
                _ => {}
            },
            "PUT" => match self {
                Command::Curl(curl) => curl.set_put_method(),
                _ => {}
            },
            "PATCH" => match self {
                Command::Curl(curl) => curl.set_patch_method(),
                _ => {}
            },
            "DELETE" => match self {
                Command::Curl(curl) => curl.set_delete_method(),

                _ => {}
            },
            _ => {}
        }
    }
    pub fn add_download_auth(&mut self, user: &str, pwd: &str) {
        if let Command::Wget(wget) = self {
            wget.add_auth(user, pwd)
        }
    }

    pub fn set_url(&mut self, url: String) {
        match self {
            Command::Curl(curl) => {
                curl.set_url(&url);
            }
            Command::Wget(wget) => {
                wget.set_url(&url);
            }
        }
    }

    pub fn set_outfile(&mut self, file: &str) {
        match self {
            Command::Curl(curl) => {
                curl.add_flag(CurlFlag::Output(
                    CurlFlagType::Output.get_value(),
                    Some(String::from(file)),
                ));
            }
            Command::Wget(wget) => {
                wget.set_output(file);
            }
        }
    }

    pub fn set_headers(&mut self, headers: Vec<String>) {
        if let Command::Curl(curl) = self {
            curl.add_headers(headers);
        }
    }

    pub fn set_auth(&mut self, auth: AuthKind) {
        match self {
            Command::Curl(curl) => match auth {
                AuthKind::Basic(login) => curl.set_basic_auth(login),
                AuthKind::Bearer(token) => curl.set_bearer_auth(token),
                AuthKind::Digest(login) => curl.set_digest_auth(&login),
                AuthKind::AwsSigv4(login) => curl.set_aws_sigv4_auth(login),
                AuthKind::Ntlm(login) => curl.set_ntlm_auth(&login),
                AuthKind::Kerberos(login) => curl.set_kerberos_auth(&login),
                AuthKind::Spnego(login) => curl.set_spnego_auth(login),
                AuthKind::NtlmWb(login) => curl.set_ntlm_wb_auth(&login),
                AuthKind::None => {}
            },
            Command::Wget(_) => {}
        }
    }

    pub fn get_response(&self) -> Option<String> {
        match self {
            Command::Curl(curl) => curl.get_response(),
            Command::Wget(wget) => wget.get_response(),
        }
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        if let Command::Curl(curl) = self {
            curl.set_verbose(verbose);
        }
    }

    pub fn execute(&mut self) -> Result<(), std::io::Error> {
        match self {
            Command::Curl(curl) => {
                curl.execute().unwrap();
                Ok(())
            }
            Command::Wget(wget) => {
                wget.execute().unwrap();
                Ok(())
            }
        }
    }

    pub fn set_rec_download_level(&mut self, level: usize) {
        if let Command::Wget(wget) = self {
            wget.set_rec_download_level(level);
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

    pub fn set_response(&mut self, response: &str) {
        if let Command::Curl(curl) = self {
            curl.set_response(response);
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

impl<'a> Default for Command<'a> {
    fn default() -> Self {
        Command::Curl(Curl::new())
    }
}
