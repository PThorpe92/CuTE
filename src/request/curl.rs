use super::ExecuteOption;
use crate::database::db::DB;
use crate::display::{menuopts::CURL, AppOptions, HeaderKind};
use curl::easy::{Auth, Easy2, Handler, List, WriteError};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::{
    fmt::{Display, Formatter},
    io::{Read, Write},
    str::FromStr,
    u8,
};
impl DerefMut for CurlHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Deref for CurlHandler {
    type Target = Easy2<Collector>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct CurlHandler(Easy2<Collector>);
#[derive(Debug, Serialize, Deserialize, Eq, Clone, PartialEq)]
pub struct Collector(Vec<u8>);
impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Curl {
    // The libcurl interface for our command/request
    #[serde(skip)]
    curl: CurlHandler,
    pub method: Method,
    auth: AuthKind,
    // The final cli command string
    cmd: String,
    headers: Option<Vec<String>>,
    url: String,
    // Build this on the App struct and pass it here to store for serialization
    pub opts: Vec<AppOptions>,
    resp: Option<String>,
    upload_file: Option<String>,
    outfile: Option<String>,
    // Whether to save the (command, auth/key) to DB after execution
    save: (bool, bool),
}

impl Default for CurlHandler {
    fn default() -> Self {
        Self(Easy2::new(Collector(Vec::new())))
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Eq, Clone, PartialEq)]
pub enum Method {
    #[default]
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
}
impl Method {
    pub fn needs_reset(&self) -> bool {
        matches!(self, Method::Put | Method::Patch | Method::Post)
    }
}
impl FromStr for Method {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "PATCH" => Ok(Method::Patch),
            "DELETE" => Ok(Method::Delete),
            "HEAD" => Ok(Method::Head),
            _ => Err(String::from("GET")),
        }
    }
}
impl Display for Method {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Method::Get => write!(f, "GET"),
            Method::Post => write!(f, "POST"),
            Method::Put => write!(f, "PUT"),
            Method::Patch => write!(f, "PATCH"),
            Method::Delete => write!(f, "DELETE"),
            Method::Head => write!(f, "HEAD"),
        }
    }
}
impl Eq for Curl {}

impl Clone for Curl {
    fn clone(&self) -> Self {
        let mut curl = Curl::new();
        let _ = self.opts.iter().map(|x| curl.add_option(x));
        curl.set_url(self.url.as_str());

        match self.method {
            Method::Get => curl.set_get_method(),
            Method::Post => curl.set_post_method(),
            Method::Put => curl.set_put_method(),
            Method::Patch => curl.set_patch_method(),
            Method::Delete => curl.set_delete_method(),
            Method::Head => curl.set_head_method(),
        }
        if let Some(ref res) = self.resp {
            curl.set_response(res.as_str());
        }

        if let Some(ref upload_file) = self.upload_file {
            curl.set_upload_file(upload_file.as_str());
        }

        if let Some(ref outfile) = self.outfile {
            curl.set_outfile(outfile);
        }
        if self.cmd != CURL {
            // our cmd string has been built
            curl.cmd = self.cmd.clone();
        }
        Self {
            curl: CurlHandler::default(),
            method: self.method.clone(),
            auth: self.auth.clone(),
            cmd: self.cmd.clone(),
            url: self.url.clone(),
            opts: self.opts.clone(),
            resp: self.resp.clone(),
            headers: self.headers.clone(),
            upload_file: self.upload_file.clone(),
            outfile: self.outfile.clone(),
            save: self.save,
        }
    }
}

impl PartialEq for Curl {
    fn eq(&self, other: &Self) -> bool {
        self.method == other.method
            && self.auth == other.auth
            && self.cmd == other.cmd
            && self.url == other.url
            && self.opts == other.opts
            && self.resp == other.resp
            && self.headers == other.headers
            && self.upload_file == other.upload_file
            && self.outfile == other.outfile
            && self.save == other.save
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, Clone, PartialEq)]
pub enum AuthKind {
    None,
    Ntlm,
    Basic(String),
    Bearer(String),
    Digest(String),
    AwsSigv4,
    Spnego,
}

impl AuthKind {
    pub fn has_token(&self) -> bool {
        matches!(
            self,
            AuthKind::Bearer(_) | AuthKind::Basic(_) | AuthKind::Digest(_)
        )
    }
    pub fn get_token(&self) -> Option<String> {
        match self {
            AuthKind::Bearer(token) => Some(token.clone()),
            AuthKind::Basic(login) => Some(login.clone()),
            AuthKind::Digest(login) => Some(login.clone()),
            _ => None,
        }
    }
}
#[rustfmt::skip]
impl Display for AuthKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            AuthKind::None            => write!(f, "None"),
            AuthKind::Ntlm            => write!(f, "NTLM"),
            AuthKind::Basic(login)    => write!(f, "Basic: {}", login),
            AuthKind::Bearer(token)   => write!(f, "Authorization: Bearer {}", token),
            AuthKind::Digest(login)   => write!(f, "Digest Auth: {}", login),
            AuthKind::AwsSigv4        => write!(f, "AWS SignatureV4"),
            AuthKind::Spnego          => write!(f, "SPNEGO Auth"),
        }
    }
}

impl Default for Curl {
    fn default() -> Self {
        Self {
            curl: CurlHandler::default(),
            method: Method::Get,
            auth: AuthKind::None,
            cmd: String::from(CURL),
            url: String::new(),
            opts: Vec::new(),
            headers: None,
            resp: None,
            upload_file: None,
            outfile: None,
            save: (false, false),
        }
    }
}
impl Curl {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_url(&self) -> &str {
        &self.url
    }
    pub fn get_method(&self) -> &Method {
        &self.method
    }
    pub fn add_basic_auth(&mut self, info: &str) {
        self.auth = AuthKind::Basic(String::from(info));
    }
    fn apply_method(&mut self) {
        match self.method {
            Method::Get => self.set_get_method(),
            Method::Post => self.set_post_method(),
            Method::Put => self.set_put_method(),
            Method::Patch => self.set_patch_method(),
            Method::Delete => self.set_delete_method(),
            Method::Head => self.set_head_method(),
        }
    }

    pub fn get_response(&self) -> Option<String> {
        self.resp.clone()
    }

    pub fn build_command_string(&mut self) {
        let mut cmd: Vec<String> = vec![self.cmd.clone()];
        cmd.push(String::from("-X"));
        cmd.push(self.method.to_string());
        cmd.push(self.url.clone());
        for flag in self.opts.iter() {
            cmd.push(flag.get_curl_flag_value());
        }
        if self.headers.is_some() {
            self.headers.as_ref().unwrap().iter().for_each(|h| {
                cmd.push(String::from("-H"));
                cmd.push(h.clone());
            });
        }
        self.cmd = cmd.join(" ").trim().to_string();
    }

    // this is only called after execution, we need to
    // find out if its been built already
    pub fn get_command_string(&mut self) -> String {
        if self.cmd == CURL {
            self.build_command_string();
        }
        self.cmd.clone()
    }

    pub fn set_outfile(&mut self, outfile: &str) {
        self.outfile = Some(String::from(outfile));
    }

    pub fn set_url(&mut self, url: &str) {
        self.url = String::from(url.trim());
        self.curl.url(url).unwrap();
    }

    pub fn has_auth(&self) -> bool {
        self.auth != AuthKind::None
    }

    pub fn set_response(&mut self, response: &str) {
        self.resp = Some(String::from(response));
    }

    pub fn set_cookie_path(&mut self, path: &str) {
        self.curl.cookie_file(path).unwrap();
    }

    pub fn set_cookie_jar(&mut self, path: &str) {
        self.curl.cookie_jar(path).unwrap();
    }
    pub fn reset_cookie_session(&mut self) {
        self.curl.cookie_session(true).unwrap();
    }

    pub fn get_upload_file(&self) -> Option<String> {
        self.upload_file.clone()
    }

    #[rustfmt::skip]
    pub fn execute(&mut self, mut db: Option<Box<&mut DB>>) -> Result<(), String> {
        let mut list = List::new();
        curl::init();
        // we do this again because if it's a patch | put and there's a
        // body, it will default to post
        self.apply_method(); 
        let mut has_headers = self.handle_auth_exec(&mut list);
        if let Some(ref headers) = self.headers {
            headers
                .iter()
                .for_each(|h| list.append(h.as_str()).unwrap());
            has_headers = true;
        }
        if self.will_save_command() {
            if let Some(ref mut db) = db {
                self.build_command_string();
                let command_string = &self.get_command_string();
                let command_json = serde_json::to_string(&self)
                    .map_err(|e| format!("Error serializing command: {}", e))?;
                if db.add_command(command_string, command_json, None).is_err() {
                    println!("Error saving command to DB");
                }
            }
        }
        // Save token to DB
        if self.will_save_token() {
            if let Some(ref mut db) = db {
                if db
                    .add_key(&self.auth.get_token().unwrap_or_default())
                    .is_err()
                {
                    println!("Error saving token to DB");
                }
            }
        }
        // Append headers if needed
        if has_headers {
            self.curl
                .http_headers(list)
                .map_err(|e| format!("Error setting headers: {:?}", e))?;
        }

        // Upload file if specified
        if let Some(ref upload_file) = self.upload_file {
            if let Ok(file) = std::fs::File::open(upload_file) {
                let mut buff: Vec<u8> = Vec::new();
                let mut reader = std::io::BufReader::new(file);
                reader
                    .read_to_end(&mut buff)
                    .map_err(|e| format!("Error reading file: {}", e))?;

                // set connect only + establish connection to the URL
                self.curl
                    .connect_only(true)
                    .map_err(|e| format!("Error connecting: {:?}", e))?;

                // Handle upload errors
                self.curl
                    .perform()
                    .map_err(|err| format!("Error making connection: {:?}", err))?;
                self.curl
                    .send(buff.as_slice())
                    .map_err(|e| format!("Error with upload: {}", e))?;
            }
        }

        // Perform the main request
        self.curl
            .perform()
            .map_err(|err| format!("Error: {:?}", err))?;
        let contents = self.curl.get_ref();
        let res = String::from_utf8_lossy(&contents.0);
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&res) {
            self.resp = Some(serde_json::to_string_pretty(&json).unwrap());
        } else {
            self.resp = Some(res.to_string());
        }
        Ok(())
    }

    pub fn set_auth(&mut self, auth: AuthKind) {
        match auth {
            AuthKind::Basic(ref info) => self.set_basic_auth(info),
            AuthKind::Ntlm => self.set_ntlm_auth(),
            AuthKind::Bearer(ref token) => self.set_bearer_auth(token),
            AuthKind::AwsSigv4 => self.set_aws_sigv4_auth(),
            AuthKind::Digest(login) => self.set_digest_auth(&login),
            AuthKind::Spnego => self.set_spnego_auth(),
            AuthKind::None => {}
        }
    }

    pub fn set_method(&mut self, method: Method) {
        match method {
            Method::Get => self.set_get_method(),
            Method::Post => self.set_post_method(),
            Method::Put => self.set_put_method(),
            Method::Patch => self.set_patch_method(),
            Method::Delete => self.set_delete_method(),
            Method::Head => self.set_head_method(),
        }
    }

    pub fn set_cert_info(&mut self, opt: bool) {
        self.curl.certinfo(opt).unwrap();
    }

    pub fn set_referrer(&mut self, referrer: &str) {
        self.curl.referer(referrer).unwrap();
    }

    pub fn set_proxy_tunnel(&mut self, opt: bool) {
        self.curl.http_proxy_tunnel(opt).unwrap();
    }

    pub fn set_verbose(&mut self, opt: bool) {
        self.curl.verbose(opt).unwrap();
    }

    pub fn set_fail_on_error(&mut self, fail: bool) {
        self.curl.fail_on_error(fail).unwrap();
    }

    pub fn set_unix_socket(&mut self, socket: &str) {
        self.curl.unix_socket(socket).unwrap();
    }

    pub fn enable_progress_bar(&mut self, on: bool) {
        self.curl.progress(on).unwrap();
    }

    pub fn set_content_header(&mut self, kind: &HeaderKind) {
        if let Some(ref mut headers) = self.headers {
            headers.push(kind.to_string());
        } else {
            self.headers = Some(vec![kind.to_string()]);
        }
    }

    pub fn save_command(&mut self, opt: bool) {
        self.save.0 = opt;
    }

    pub fn add_headers(&mut self, headers: &str) {
        if self.headers.is_some() {
            self.headers.as_mut().unwrap().push(headers.to_string());
        } else {
            self.headers = Some(vec![headers.to_string()]);
        }
    }

    pub fn save_token(&mut self, opt: bool) {
        self.save.1 = opt;
    }

    pub fn get_token(&self) -> Option<String> {
        self.auth.get_token()
    }

    pub fn remove_headers(&mut self, headers: &str) {
        if self.headers.is_some() {
            self.headers
                .as_mut()
                .unwrap()
                .retain(|x| !headers.contains(x));
        }
    }
    pub fn match_wildcard(&mut self, opt: bool) {
        self.curl.wildcard_match(opt).unwrap();
    }

    pub fn set_unrestricted_auth(&mut self, opt: bool) {
        self.curl.unrestricted_auth(opt).unwrap();
    }

    pub fn set_user_agent(&mut self, ua: &str) {
        self.curl.useragent(ua).unwrap();
    }

    pub fn set_max_redirects(&mut self, redirects: usize) {
        self.curl
            .max_redirections(redirects as u32)
            .unwrap_or_default();
    }

    pub fn set_ca_path(&mut self, ca_path: &str) {
        self.curl.cainfo(ca_path).unwrap_or_default();
    }

    pub fn set_tcp_keepalive(&mut self, opt: bool) {
        self.curl.tcp_keepalive(opt).unwrap_or_default();
    }

    pub fn set_request_body(&mut self, body: &str) {
        self.curl
            .post_fields_copy(body.as_bytes())
            .unwrap_or_default();
    }

    pub fn set_follow_redirects(&mut self, opt: bool) {
        self.curl.follow_location(opt).unwrap_or_default();
    }

    pub fn add_cookie(&mut self, cookie: &str) {
        self.curl.cookie(cookie).unwrap_or_default();
    }

    pub fn set_upload_file(&mut self, file: &str) {
        self.upload_file = Some(file.to_string());
        self.curl.upload(true).unwrap_or_default();
    }

    pub fn write_output(&mut self) -> Result<(), std::io::Error> {
        println!("{}", self.outfile.as_ref().unwrap().clone());
        match self.outfile {
            Some(ref mut outfile) => {
                let mut file = match std::fs::File::create(outfile) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("Error creating file: {:?}", e);
                        return Err(e);
                    }
                };
                let mut writer = std::io::BufWriter::new(&mut file);

                if let Some(resp) = &self.resp {
                    if let Err(e) = writer.write_all(resp.as_bytes()) {
                        eprintln!("Error writing to file: {:?}", e);
                        return Err(e);
                    }
                }

                Ok(())
            }
            None => Ok(()),
        }
    }
    pub fn enable_response_headers(&mut self, opt: bool) {
        self.curl.show_header(opt).unwrap_or_default();
    }

    fn will_save_token(&self) -> bool {
        // (0: save_command, 1: save_token)
        self.save.1
    }

    pub fn easy_from_opts(&mut self) {
        self.build_command_string();
        self.curl.url(&self.url).unwrap();
        self.apply_method();
        let opts = self.opts.clone();
        for opt in opts.iter() {
            self.add_option(opt);
        }
    }
    pub fn set_any_auth(&mut self) {
        let _ = self.curl.http_auth(&Auth::new());
    }

    pub fn set_basic_auth(&mut self, login: &str) {
        self.auth = AuthKind::Basic(String::from(login));
    }

    pub fn set_head_method(&mut self) {
        self.method = Method::Head;
        self.curl.nobody(true).unwrap();
    }

    pub fn set_digest_auth(&mut self, login: &str) {
        self.auth = AuthKind::Digest(String::from(login));
    }

    pub fn set_aws_sigv4_auth(&mut self) {
        self.auth = AuthKind::AwsSigv4;
    }

    pub fn set_spnego_auth(&mut self) {
        self.auth = AuthKind::Spnego;
    }

    pub fn will_save_command(&self) -> bool {
        // (0: save_command, 1: save_token)
        self.save.0
    }

    pub fn set_get_method(&mut self) {
        self.method = Method::Get;
        self.curl.get(true).unwrap();
    }

    pub fn set_post_method(&mut self) {
        self.method = Method::Post;
        self.curl.post(true).unwrap();
    }

    pub fn set_put_method(&mut self) {
        self.method = Method::Put;
        self.curl.put(true).unwrap();
    }

    pub fn set_patch_method(&mut self) {
        self.method = Method::Patch;
        self.curl.custom_request("PATCH").unwrap();
    }

    pub fn set_delete_method(&mut self) {
        self.method = Method::Delete;
        self.curl.custom_request("DELETE").unwrap();
    }

    pub fn set_ntlm_auth(&mut self) {
        self.auth = AuthKind::Ntlm;
    }

    pub fn set_bearer_auth(&mut self, token: &str) {
        self.auth = AuthKind::Bearer(String::from(token));
    }

    pub fn show_headers(&mut self) {
        self.curl.show_header(true).unwrap();
    }

    fn handle_auth_exec(&mut self, list: &mut List) -> bool {
        match &self.auth {
            AuthKind::None => {}
            AuthKind::Basic(login) => {
                self.curl
                    .username(login.split(':').next().unwrap())
                    .unwrap();
                self.curl
                    .password(login.split(':').last().unwrap())
                    .unwrap();
                let _ = self.curl.http_auth(Auth::new().basic(true));
            }
            AuthKind::Bearer(ref token) => {
                list.append(&format!("Authorization: {token}")).unwrap();
                return true;
            }
            AuthKind::Digest(login) => {
                self.curl
                    .username(login.split(':').next().unwrap())
                    .unwrap();
                self.curl
                    .password(login.split(':').last().unwrap())
                    .unwrap();
                let _ = self.curl.http_auth(Auth::new().digest(true));
            }
            AuthKind::Ntlm => {
                let _ = self.curl.http_auth(Auth::new().ntlm(true));
            }
            AuthKind::Spnego => {
                let _ = self.curl.http_auth(Auth::new().gssnegotiate(true));
            }
            AuthKind::AwsSigv4 => {
                let _ = self.curl.http_auth(Auth::new().aws_sigv4(true));
            }
        }
        false
    }

    pub fn url_encode(&mut self, data: &str) {
        self.url = self.curl.url_encode(data.as_bytes());
    }
}
