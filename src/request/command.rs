use super::curl::{AuthKind, CurlFlagType};
use crate::request::curl::{Curl, CurlFlag};
use crate::request::wget::Wget;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Command<'a> {
    Curl(Curl<'a>),
    Wget(Wget),
}
impl<'a> Deref for Command<'a> {
    type Target = Curl<'a>;
    fn deref(&self) -> &Self::Target {
        match self {
            Command::Curl(curl) => curl,
            _ => panic!("cannot deref wget commands"),
        }
    }
}

impl<'a> DerefMut for Command<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Command::Curl(curl) => curl,
            _ => panic!("cannot deref wget commands"),
        }
    }
}

impl<'a> Command<'a> {
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

    pub fn set_url(&mut self, url: &str) {
        match self {
            Command::Curl(curl) => {
                curl.set_url(&url);
            }
            Command::Wget(wget) => {
                wget.set_url(&url);
            }
        }
    }

    pub fn get_url(&self) -> String {
        match self {
            Command::Curl(curl) => curl.get_url(),
            Command::Wget(wget) => wget.get_url(),
        }
    }

    pub fn set_outfile(&mut self, file: &str) {
        match self {
            Command::Curl(curl) => {
                curl.set_outfile(String::from(file));
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

    pub fn save_token(&mut self) {
        if let Command::Curl(curl) = self {
            curl.save_token(!curl.will_save_token());
        }
    }

    pub fn set_rec_download_level(&mut self, level: usize) {
        if let Command::Wget(wget) = self {
            wget.set_rec_download_level(level);
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Command::Curl(_) => "Curl",
            Command::Wget(_) => "Wget",
        }
        .to_string()
    }

    pub fn write_response(&mut self) -> Result<(), String> {
        match self {
            Command::Curl(ref mut curl) => match curl.write_output() {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            },
            _ => Err(String::from("downloads output is written by default")),
        }
    }
}

impl<'a> Default for Command<'a> {
    fn default() -> Self {
        Command::Curl(Curl::new())
    }
}
