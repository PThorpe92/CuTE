use crate::database::db::DB;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use curl::easy::{Auth, Easy2, Handler, List, WriteError};
use std::io::Read;
use std::u8;
use std::{
    fmt::{Display, Formatter},
    io::Write,
};

use super::command::{CmdOpts, CurlOpts, CMD};

pub static DEFAULT_CMD: &str = "curl";

#[derive(Debug, Serialize, Deserialize, Eq, Clone, PartialEq)]
struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

#[derive(Debug)]
pub struct Curl<'a> {
    // The libcurl interface for our command/request
    curl: Easy2<Collector>,
    // The method type
    method: Option<Method>,
    // The auth type we will use
    auth: AuthKind,
    // The final command string we will run
    cmd: String,
    // The strings of headers
    headers: Option<Vec<String>>,
    // The url we will send the request to
    url: String,
    // The opts we will build incrementally and store
    opts: Vec<CurlFlag<'a>>,
    // The response we get back from the command if not sent to file
    resp: Option<String>,
    // Filepath of file to be uploaded
    upload_file: Option<String>,
    // Filepath of the response output file or download
    outfile: Option<String>,
    // Whether to save the (command, auth/key) to DB after execution
    save: (bool, bool),
}

#[derive(Debug, Serialize, Deserialize, Eq, Clone, PartialEq)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
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
impl<'a> Eq for Curl<'a> {}

impl<'a> Serialize for Curl<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize all fields except 'curl::Easy'
        let mut state = serializer.serialize_struct("Curl", 9)?;
        state.serialize_field("method", &self.method)?;
        state.serialize_field("auth", &self.auth)?;
        state.serialize_field("cmd", &self.cmd)?;
        state.serialize_field("headers", &self.headers)?;
        state.serialize_field("url", &self.url)?;
        state.serialize_field("opts", &self.opts)?;
        state.serialize_field("resp", &self.resp)?;
        state.serialize_field("upload_file", &self.upload_file)?;
        state.serialize_field("outfile", &self.outfile)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Curl<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize all fields except 'curl::Easy'
        struct CurlVisitor;

        impl<'de> Visitor<'de> for CurlVisitor {
            type Value = Curl<'de>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Curl")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut method = None;
                let mut auth = None;
                let mut cmd = None;
                let mut headers = None;
                let mut url = None;
                let mut opts = None;
                let mut resp = None;
                let mut upload_file = None;
                let mut outfile = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "method" => method = Some(map.next_value()?),
                        "auth" => auth = Some(map.next_value()?),
                        "cmd" => cmd = Some(map.next_value()?),
                        "headers" => headers = Some(map.next_value()?),
                        "url" => url = Some(map.next_value()?),
                        "opts" => opts = Some(map.next_value()?),
                        "resp" => resp = Some(map.next_value()?),
                        "upload_file" => upload_file = Some(map.next_value()?),
                        "outfile" => outfile = Some(map.next_value()?),
                        &_ => {}
                    }
                }
                let curl = Easy2::new(Collector(Vec::new()));
                let mut res = Curl {
                    curl,
                    method: method.ok_or_else(|| serde::de::Error::missing_field("method"))?,
                    auth: auth.ok_or_else(|| serde::de::Error::missing_field("auth"))?,
                    cmd: cmd.ok_or_else(|| serde::de::Error::missing_field("cmd"))?,
                    headers: headers.ok_or_else(|| serde::de::Error::missing_field("headers"))?,
                    url: url.ok_or_else(|| serde::de::Error::missing_field("url"))?,
                    opts: opts.ok_or_else(|| serde::de::Error::missing_field("opts"))?,
                    resp: resp.ok_or_else(|| serde::de::Error::missing_field("resp"))?,
                    upload_file: upload_file
                        .ok_or_else(|| serde::de::Error::missing_field("upload_file"))?,
                    outfile: outfile.ok_or_else(|| serde::de::Error::missing_field("outfile"))?,
                    save: (false, false),
                };
                res.easy_from_opts();
                Ok(res)
            }
        }
        deserializer.deserialize_struct(
            "Curl",
            &[
                "curl",
                "method",
                "auth",
                "cmd",
                "headers",
                "url",
                "opts",
                "resp",
                "upload_file",
                "outfile",
                "save",
            ],
            CurlVisitor,
        )
    }
}

impl<'a> Clone for Curl<'a> {
    fn clone(&self) -> Self {
        let mut curl = Curl::new();
        let _ = self.opts.iter().map(|x| curl.add_flag(x.clone()));
        curl.set_url(self.url.as_str());

        match self.method {
            Some(Method::Get) => curl.set_get_method(),
            Some(Method::Post) => curl.set_post_method(),
            Some(Method::Put) => curl.set_put_method(),
            Some(Method::Patch) => curl.set_patch_method(),
            Some(Method::Delete) => curl.set_delete_method(),
            Some(Method::Head) => curl.set_head_method(),
            None => {}
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
        if self.cmd != DEFAULT_CMD {
            // our cmd string has been built
            curl.cmd = self.cmd.clone();
        }
        curl.build_command_str();
        Self {
            curl: Easy2::new(Collector(Vec::new())),
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

impl<'a> PartialEq for Curl<'a> {
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
    Ntlm(String),
    Basic(String),
    Bearer(String),
    Digest(String),
    AwsSigv4(String),
    Spnego(String),
    NtlmWb(String),
    Kerberos(String),
}

impl AuthKind {
    pub fn get_token(&self) -> Option<String> {
        match self {
            AuthKind::Bearer(token) => Some(token.clone()),
            AuthKind::Basic(login) => Some(login.clone()),
            AuthKind::Digest(login) => Some(login.clone()),
            AuthKind::AwsSigv4(login) => Some(login.clone()),
            AuthKind::Spnego(login) => Some(login.clone()),
            AuthKind::Ntlm(login) => Some(login.clone()),
            AuthKind::NtlmWb(login) => Some(login.clone()),
            AuthKind::Kerberos(login) => Some(login.clone()),
            _ => None,
        }
    }
}

impl Display for AuthKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            AuthKind::None => write!(f, "None"),
            AuthKind::Ntlm(login) => write!(f, "NTLM: {}", login),
            AuthKind::Basic(login) => write!(f, "Basic: {}", login),
            AuthKind::Bearer(token) => write!(f, "Bearer: {}", token),
            AuthKind::Digest(login) => write!(f, "Digest: {}", login),
            AuthKind::AwsSigv4(login) => write!(f, "AWS SignatureV4: {}", login),
            AuthKind::Spnego(login) => write!(f, "SPNEGO: {}", login),
            AuthKind::NtlmWb(login) => write!(f, "NTLM WB: {}", login),
            AuthKind::Kerberos(login) => write!(f, "Kerberos: {}", login),
        }
    }
}

impl<'a> CMD for Curl<'a> {}

impl<'a> Default for Curl<'a> {
    fn default() -> Self {
        Self {
            curl: Easy2::new(Collector(Vec::new())),
            method: None,
            auth: AuthKind::None,
            cmd: String::from(DEFAULT_CMD),
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
impl<'a> CmdOpts for Curl<'a> {
    fn get_url(&self) -> String {
        self.url.clone()
    }
    fn add_basic_auth(&mut self, info: &str) {
        self.add_flag(CurlFlag::Basic(
            CurlFlagType::Basic.get_value(),
            Some(String::from(info)),
        ));
        self.auth = AuthKind::Basic(String::from(info));
    }
    //
    // this is only called after execution, we need to
    // find out if its been built already
    fn get_command_string(&mut self) -> String {
        self.build_command_str();
        self.cmd.clone()
    }

    fn set_outfile(&mut self, outfile: &str) {
        self.add_flag(CurlFlag::Output(CurlFlagType::Output.get_value(), None));
        self.outfile = Some(String::from(outfile));
    }

    fn set_url(&mut self, url: &str) {
        self.url = String::from(url);
        self.curl.url(url).unwrap();
    }

    fn has_auth(&self) -> bool {
        self.auth != AuthKind::None
    }

    fn set_response(&mut self, response: &str) {
        self.resp = Some(String::from(response));
    }

    fn set_rec_download_level(&mut self, _level: usize) {}

    fn get_response(&self) -> String {
        self.resp.clone().unwrap_or_default()
    }

    fn execute(&mut self, mut db: Option<&mut Box<DB>>) -> Result<(), String> {
        let mut list = List::new();
        // Setup auth if we have it, will return whether we appended to the list
        let mut has_headers = self.handle_auth_exec(&mut list);
        if self.headers.is_some() {
            has_headers = true;
            self.headers
                .as_ref()
                .unwrap()
                .iter()
                .for_each(|h| list.append(h.as_str()).unwrap());
        }
        if self.will_save_command() {
            println!("Saving command");
            let _ = db.as_mut().unwrap().add_command(
                &self.get_command_string(),
                serde_json::to_string(&self).unwrap_or(String::from("Error serializing command")),
            );
        }
        if self.will_save_token() {
            let _ = db
                .unwrap()
                .add_key(&self.auth.get_token().unwrap_or_default());
        }
        // We have to append the list of headers all at once
        // but if we never appended to the list, we skip this
        if has_headers {
            self.curl.http_headers(list).unwrap();
        }

        // If we are uploading a file...
        if let Some(ref upload_file) = self.upload_file {
            let file = std::fs::File::open(upload_file).unwrap();
            let mut buff: Vec<u8> = Vec::new();
            let mut reader = std::io::BufReader::new(file);
            let _ = reader.read_to_end(&mut buff);
            // set connect only + establish connection to the URL
            self.curl.connect_only(true).unwrap();
            if self.curl.perform().is_ok() {
                // Upload the file contents
                if self.curl.send(buff.as_slice()).is_ok() {
                    Ok(())
                } else {
                    Err(String::from("Error with upload"))
                }
            } else {
                Err(String::from("Error making connection"))
            }
        } else {
            self.curl.perform().unwrap();
            let contents = self.curl.get_ref();
            let res = String::from_utf8_lossy(&contents.0);
            if let Ok(json) =
                serde_json::from_str::<serde_json::Value>(&String::from_utf8_lossy(&contents.0))
            {
                self.resp = Some(serde_json::to_string_pretty(&json).unwrap());
                Ok(())
            } else {
                self.resp = Some(res.to_string());
                Ok(())
            }
        }
    }
}
impl<'a> CurlOpts for Curl<'a> {
    fn set_auth(&mut self, auth: AuthKind) {
        match auth {
            AuthKind::Basic(info) => self.set_basic_auth(info),
            AuthKind::Ntlm(info) => self.set_ntlm_auth(&info),
            AuthKind::Bearer(token) => self.set_bearer_auth(token),
            AuthKind::AwsSigv4(login) => self.set_aws_sigv4_auth(login),
            AuthKind::Digest(login) => self.set_digest_auth(&login),
            AuthKind::Kerberos(info) => self.set_kerberos_auth(&info),
            AuthKind::NtlmWb(info) => self.set_ntlm_wb_auth(&info),
            AuthKind::Spnego(info) => self.set_spnego_auth(info),
            AuthKind::None => {}
        }
    }

    fn has_unix_socket(&self) -> bool {
        let flag = &CurlFlag::UnixSocket(CurlFlagType::UnixSocket.get_value(), None);
        self.has_flag(flag)
    }
    fn set_method(&mut self, method: String) {
        match method.as_str() {
            "GET" => self.set_get_method(),
            "POST" => self.set_post_method(),
            "PUT" => self.set_put_method(),
            "PATCH" => self.set_patch_method(),
            "DELETE" => self.set_delete_method(),
            "HEAD" => self.set_head_method(),
            _ => {}
        }
    }
    fn set_cert_info(&mut self, opt: bool) {
        let flag = CurlFlag::CertInfo(CurlFlagType::CertInfo.get_value(), None);
        self.toggle_flag(&flag);
        self.curl.certinfo(opt).unwrap();
    }

    fn set_referrer(&mut self, referrer: &str) {
        let flag = CurlFlag::Referrer(CurlFlagType::Referrer.get_value(), None);
        self.toggle_flag(&flag);
        self.curl.referer(referrer).unwrap();
    }

    fn set_proxy_tunnel(&mut self, opt: bool) {
        let flag = CurlFlag::ProxyTunnel(CurlFlagType::ProxyTunnel.get_value(), None);
        self.toggle_flag(&flag);
        self.curl.http_proxy_tunnel(opt).unwrap();
    }

    fn will_save_command(&self) -> bool {
        self.save.0
    }

    fn set_verbose(&mut self, opt: bool) {
        let flag = CurlFlag::Verbose(CurlFlagType::Verbose.get_value(), None);
        self.toggle_flag(&flag);
        self.curl.verbose(opt).unwrap();
    }

    fn set_fail_on_error(&mut self, fail: bool) {
        let flag = CurlFlag::FailOnError(CurlFlagType::FailOnError.get_value(), None);
        self.toggle_flag(&flag);
        self.curl.fail_on_error(fail).unwrap();
    }

    fn set_unix_socket(&mut self, socket: &str) {
        let flag = CurlFlag::UnixSocket(CurlFlagType::UnixSocket.get_value(), None);
        self.toggle_flag(&flag);
        self.curl.unix_socket(socket).unwrap();
    }

    fn enable_progress_bar(&mut self, on: bool) {
        let flag = CurlFlag::Progress(CurlFlagType::Progress.get_value(), None);
        self.toggle_flag(&flag);
        self.curl.progress(on).unwrap();
    }

    fn save_command(&mut self, opt: bool) {
        self.save.0 = opt;
    }

    fn add_headers(&mut self, headers: String) {
        if self.headers.is_some() {
            self.headers.as_mut().unwrap().push(headers);
        } else {
            self.headers = Some(vec![headers]);
        }
    }

    fn save_token(&mut self, opt: bool) {
        self.save.1 = opt;
    }

    fn get_token(&self) -> Option<String> {
        self.auth.get_token()
    }

    fn remove_headers(&mut self, headers: String) {
        if self.headers.is_some() {
            self.headers
                .as_mut()
                .unwrap()
                .retain(|x| !headers.contains(x));
        }
    }
    fn match_wildcard(&mut self, opt: bool) {
        let flag = CurlFlag::MatchWildcard(CurlFlagType::MatchWildcard.get_value(), None);
        self.toggle_flag(&flag);
        self.curl.wildcard_match(opt).unwrap();
    }

    fn set_unrestricted_auth(&mut self, opt: bool) {
        let flag = CurlFlag::AnyAuth(CurlFlagType::AnyAuth.get_value(), None);
        self.toggle_flag(&flag);
        self.curl.unrestricted_auth(opt).unwrap();
    }

    fn set_user_agent(&mut self, ua: &str) {
        let flag = CurlFlag::User(CurlFlagType::User.get_value(), None);
        self.toggle_flag(&flag);
        self.curl.useragent(ua).unwrap();
    }

    fn set_max_redirects(&mut self, redirects: usize) {
        let flag = CurlFlag::MaxRedirects(
            CurlFlagType::MaxRedirects.get_value(),
            Some(redirects.to_string()),
        );
        self.toggle_flag(&flag);
        self.curl.max_redirections(redirects as u32).unwrap();
    }

    fn set_ca_path(&mut self, ca_path: &str) {
        let flag = CurlFlag::CaPath(CurlFlagType::CaPath.get_value(), None);
        self.toggle_flag(&flag);
        self.curl.cainfo(ca_path).unwrap();
    }

    fn set_tcp_keepalive(&mut self, opt: bool) {
        let flag = CurlFlag::TcpKeepAlive(CurlFlagType::TcpKeepAlive.get_value(), None);
        self.toggle_flag(&flag);
        self.curl.tcp_keepalive(opt).unwrap();
    }

    fn set_follow_redirects(&mut self, opt: bool) {
        let flag = CurlFlag::FollowRedirects(CurlFlagType::FollowRedirects.get_value(), None);
        self.toggle_flag(&flag);
        self.curl.follow_location(opt).unwrap();
    }

    fn add_cookie(&mut self, cookie: String) {
        // we are removing it
        let flag = CurlFlag::Cookie(CurlFlagType::Cookie.get_value(), Some(cookie.clone()));
        self.toggle_flag(&flag);
        self.curl.cookie(cookie.as_str()).unwrap();
    }

    fn set_upload_file(&mut self, file: &str) {
        self.add_flag(CurlFlag::UploadFile(
            CurlFlagType::UploadFile.get_value(),
            Some(file.to_string()),
        ));
        self.upload_file = Some(file.to_string());
        self.curl.upload(true).unwrap();
    }

    fn write_output(&mut self) -> Result<(), std::io::Error> {
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

    fn enable_response_headers(&mut self, opt: bool) {
        self.curl.show_header(opt).unwrap();
    }
}

impl<'a> Curl<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    fn will_save_token(&self) -> bool {
        // (0: save_command, 1: save_token)
        self.save.1
    }

    fn has_flag(&self, flag: &CurlFlag<'a>) -> bool {
        self.opts
            .iter()
            .filter(|has| std::mem::discriminant(*has) == std::mem::discriminant(flag))
            .count()
            > 0
    }

    // This is a hack because when we deseialize from the DB, we get a curl struct with no curl::Easy
    // field, so we have to manually add, then set the options one at a time from the opts vector.
    // ANY time we get a command from the database to run, we have to call this method first.
    pub fn easy_from_opts(&mut self) {
        let opts = self.opts.clone();
        let url = self.url.clone();
        self.set_url(&url);
        if let Some(ref method) = self.method {
            match method {
                Method::Get => self.set_get_method(),
                Method::Post => self.set_post_method(),
                Method::Put => self.set_put_method(),
                Method::Patch => self.set_patch_method(),
                Method::Delete => self.set_delete_method(),
                Method::Head => self.curl.nobody(true).unwrap(),
            }
        }
        for opt in opts {
            match opt {
                CurlFlag::Verbose(..) => self.set_verbose(true),
                CurlFlag::Headers(_, val) => self.add_headers(val.unwrap_or(String::new())),
                CurlFlag::Output(..) => {
                    if let Some(val) = opt.get_arg() {
                        self.set_outfile(&val);
                    }
                }
                CurlFlag::Cookie(..) => {
                    if let Some(val) = opt.get_arg() {
                        self.add_cookie(val);
                    }
                }
                CurlFlag::MatchWildcard(..) => {
                    self.curl.wildcard_match(true).unwrap();
                }
                CurlFlag::User(..) => {}
                CurlFlag::Bearer(..) => {
                    if let Some(val) = opt.get_arg() {
                        self.set_bearer_auth(val);
                    }
                }
                CurlFlag::Digest(..) => {
                    if let Some(val) = opt.get_arg() {
                        self.set_digest_auth(val.as_str());
                    }
                }
                CurlFlag::Basic(..) => {
                    if let Some(val) = opt.get_arg() {
                        self.set_basic_auth(val);
                    }
                }
                CurlFlag::AnyAuth(..) => self.set_any_auth(),
                CurlFlag::Ntlm(..) => {
                    if let Some(val) = opt.get_arg() {
                        self.set_ntlm_auth(val.as_str());
                    }
                }
                CurlFlag::NtlmWb(..) => {
                    if let Some(val) = opt.get_arg() {
                        self.set_ntlm_wb_auth(val.as_str());
                    }
                }
                CurlFlag::AwsSigv4(..) => {
                    if let Some(val) = opt.get_arg() {
                        self.set_aws_sigv4_auth(val);
                    }
                }
                CurlFlag::UnixSocket(..) => {
                    if let Some(val) = opt.get_arg() {
                        self.set_unix_socket(val.as_str());
                    }
                }
                CurlFlag::UploadFile(..) => {
                    if let Some(val) = opt.get_arg() {
                        self.set_upload_file(val.as_str());
                    }
                }
                CurlFlag::SpnegoAuth(..) => {
                    if let Some(val) = opt.get_arg() {
                        self.set_spnego_auth(val);
                    }
                }
                CurlFlag::Kerberos(..) => {
                    if let Some(val) = opt.get_arg() {
                        self.set_kerberos_auth(val.as_str());
                    }
                }
                CurlFlag::DumpHeaders(..) => {
                    if let Some(val) = opt.get_arg() {
                        self.show_headers(val.as_str());
                    }
                }
                CurlFlag::CaPath(..) => {
                    if let Some(val) = opt.get_arg() {
                        self.set_ca_path(val.as_str());
                    }
                }
                CurlFlag::MaxRedirects(..) => {
                    if let Some(val) = opt.get_arg() {
                        self.set_max_redirects(val.parse::<usize>().unwrap());
                    }
                }
                CurlFlag::CertInfo(..) => self.set_cert_info(true),
                CurlFlag::FailOnError(..) => self.set_fail_on_error(true),
                CurlFlag::Proxy(..) => {}
                CurlFlag::ProxyTunnel(..) => self.set_proxy_tunnel(true),
                CurlFlag::File(..) => {}
                CurlFlag::DataUrlEncode(..) => {}
                CurlFlag::Referrer(..) => {}
                CurlFlag::FollowRedirects(..) => self.set_follow_redirects(true),
                CurlFlag::TcpKeepAlive(..) => self.set_tcp_keepalive(true),
                CurlFlag::PreventDefaultConfig(..) => {}
                CurlFlag::Progress(..) => {}
            }
        }
    }

    pub fn remove_flag(&mut self, flag: &CurlFlag<'a>) {
        self.opts
            .retain(|x| std::mem::discriminant(x) != std::mem::discriminant(flag));
    }

    pub fn set_any_auth(&mut self) {
        self.add_flag(CurlFlag::AnyAuth(CurlFlagType::AnyAuth.get_value(), None));
        let _ = self.curl.http_auth(&Auth::new());
    }

    pub fn set_basic_auth(&mut self, login: String) {
        self.add_flag(CurlFlag::Basic(
            CurlFlagType::Basic.get_value(),
            Some(login.to_string()),
        ));
        self.auth = AuthKind::Basic(login);
    }

    pub fn toggle_flag(&mut self, flag: &CurlFlag<'a>) {
        if self.has_flag(flag) {
            self.remove_flag(flag);
        } else {
            self.opts.push(flag.clone());
        }
    }

    pub fn set_head_method(&mut self) {
        if self.method.is_some() {
            self.curl.reset();
        }
        self.method = Some(Method::Head);
        self.curl.nobody(true).unwrap();
    }

    pub fn set_digest_auth(&mut self, info: &str) {
        self.add_flag(CurlFlag::Digest(
            CurlFlagType::Digest.get_value(),
            Some(info.to_string()),
        ));
        self.auth = AuthKind::Digest(info.to_string());
    }

    pub fn set_aws_sigv4_auth(&mut self, login: String) {
        self.add_flag(CurlFlag::AwsSigv4(
            CurlFlagType::AwsSigv4.get_value(),
            Some(login.clone()),
        ));
        self.auth = AuthKind::AwsSigv4(login);
    }

    pub fn set_spnego_auth(&mut self, login: String) {
        self.add_flag(CurlFlag::SpnegoAuth(
            CurlFlagType::SpnegoAuth.get_value(),
            Some(login.clone()),
        ));
        self.auth = AuthKind::Spnego(login);
    }

    pub fn will_save_command(&self) -> bool {
        // (0: save_command, 1: save_token)
        self.save.0
    }

    pub fn set_get_method(&mut self) {
        self.method = Some(Method::Get);
        self.curl.get(true).unwrap();
    }

    pub fn set_post_method(&mut self) {
        self.method = Some(Method::Post);
        self.curl.post(true).unwrap();
    }

    pub fn set_put_method(&mut self) {
        self.method = Some(Method::Put);
        self.curl.put(true).unwrap();
    }

    pub fn set_patch_method(&mut self) {
        self.method = Some(Method::Patch);
        self.curl.custom_request("PATCH").unwrap();
    }

    pub fn set_delete_method(&mut self) {
        self.method = Some(Method::Delete);
        self.curl.custom_request("DELETE").unwrap();
    }

    pub fn set_ntlm_auth(&mut self, login: &str) {
        self.add_flag(CurlFlag::Ntlm(
            CurlFlagType::Ntlm.get_value(),
            Some(login.to_string()),
        ));
        self.auth = AuthKind::Ntlm(login.to_string());
    }

    pub fn set_ntlm_wb_auth(&mut self, login: &str) {
        self.add_flag(CurlFlag::NtlmWb(
            CurlFlagType::NtlmWb.get_value(),
            Some(login.to_string()),
        ));
        self.auth = AuthKind::NtlmWb(login.to_string());
    }

    pub fn set_bearer_auth(&mut self, token: String) {
        self.add_flag(CurlFlag::Bearer(
            CurlFlagType::Bearer.get_value(),
            Some(format!("Authorization: Bearer {token}")),
        ));
        self.auth = AuthKind::Bearer(token);
    }

    pub fn show_headers(&mut self, file: &str) {
        self.add_flag(CurlFlag::DumpHeaders(
            CurlFlagType::DumpHeaders.get_value(),
            Some(file.to_string()),
        ));
        self.curl.show_header(true).unwrap();
    }

    pub fn set_kerberos_auth(&mut self, login: &str) {
        self.add_flag(CurlFlag::Kerberos(
            CurlFlagType::Kerberos.get_value(),
            Some(login.to_string()),
        ));
        self.auth = AuthKind::Kerberos(login.to_string());
    }

    fn build_command_str(&mut self) {
        let mut cmd: Vec<String> = vec![self.cmd.clone()];
        if let Some(ref method) = &self.method {
            cmd.push(String::from("-X"));
            cmd.push(method.to_string());
        }
        cmd.push(self.url.clone());
        for flag in &self.opts {
            cmd.push(flag.get_value().to_string());
            if let Some(arg) = &flag.get_arg() {
                cmd.push(arg.to_owned());
            }
        }
        if self.headers.is_some() {
            self.headers.as_ref().unwrap().iter().for_each(|h| {
                cmd.push(String::from("-H"));
                cmd.push(h.clone());
            });
        }
        self.cmd = cmd.join(" ").trim().to_string();
    }

    pub fn handle_auth_exec(&mut self, list: &mut List) -> bool {
        // we need to know if we have appended to this list
        let mut list_edited = false;
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
            AuthKind::Bearer(token) => {
                list_edited = true;
                let _ = list.append(&format!("Authorization: Bearer {}", token.clone()));
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
            AuthKind::Ntlm(login) => {
                self.curl
                    .username(login.split(':').next().unwrap())
                    .unwrap();
                self.curl
                    .password(login.split(':').last().unwrap())
                    .unwrap();
                let _ = self.curl.http_auth(Auth::new().ntlm(true));
            }
            AuthKind::NtlmWb(login) => {
                self.curl.username(login).unwrap();
                let _ = self.curl.http_auth(Auth::new().ntlm_wb(true));
            }
            AuthKind::Spnego(login) => {
                self.curl.username(login).unwrap();
                let _ = self.curl.http_auth(Auth::new().gssnegotiate(true));
            }
            AuthKind::AwsSigv4(login) => {
                self.curl.username(login).unwrap();
                let _ = self.curl.http_auth(Auth::new().aws_sigv4(true));
            }
            _ => {}
        };
        list_edited
    }

    pub fn url_encode(&mut self, data: &str) {
        self.add_flag(CurlFlag::DataUrlEncode(
            CurlFlagType::DataUrlEncode.get_value(),
            Some(data.to_string()),
        ));
        self.url = self.curl.url_encode(data.as_bytes());
    }

    pub fn add_flag(&mut self, flag: CurlFlag<'a>) {
        self.opts.push(flag);
    }
}

// curl.opts  =  Vec<Flag>  =  vec!["--cert-type", "PEM"] so flag / argument
// but we dont want to have to provide/remember the "-X"(flag) so we store it in the enum
// We may have "--verbose" which is a flag with no value
// But each enum variant has the default flag stored as a static string, so we can use that
// to build the command incrementally by just providing the argument value when we create the flag.
#[macro_export]
macro_rules! define_curl_flags {
    (
        $( $variant:ident($default:expr), )*
    ) => {
        #[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
        pub enum CurlFlag<'a> {
            $( $variant(&'a str, Option<String>), )*
        }

        impl<'a> CurlFlag<'a> {
            pub fn new(flag: CurlFlagType, value: Option<String>) -> Self {
                match flag {
                    $( CurlFlagType::$variant => CurlFlag::$variant(flag.get_value(), value),)*
                }
            }

            pub fn get_arg(&self) -> Option<String> {
                match self {
                    $( CurlFlag::$variant(_, arg) => arg.clone(), )*
                }
            }

            pub fn get_value(&self) -> &'a str {
                match self {
                    $( CurlFlag::$variant(flag, _) => flag, )*
                }
            }
        }

        #[derive(Debug, Eq, Copy, Clone, PartialEq)]
        pub enum CurlFlagType {
            $( $variant, )*
        }
        impl CurlFlagType {
            pub fn get_value(&self) -> &'static str {
            match self {
                $( CurlFlagType::$variant => $default, )*
            }
            }
        }
    };
}

// Define the CurlFlag enum using the macro.
define_curl_flags! {
    Verbose("-v"),
    Cookie("-b"),
    Output("-o"),
    User("-u"),
    Bearer("-H"),
    CertInfo("--certinfo"),
    Headers("-H"),
    Digest("--digest"),
    Basic("-H"),
    AnyAuth("--any-auth"),
    UnixSocket("--unix-socket"),
    UploadFile("--upload-file"),
    Ntlm("--ntlm"),
    NtlmWb("--ntlm-wb"),
    Proxy("-x"),
    AwsSigv4("--aws-sigv4"),
    ProxyTunnel("--proxy-tunnel"),
    File("-F"),
    DataUrlEncode("--data-urlencode"),
    DumpHeaders("--dump-headers"),
    Referrer("-e"),
    MatchWildcard("--glob"),
    FailOnError("--fail"),
    FollowRedirects("-L"),
    MaxRedirects("--max-redirs"),
    PreventDefaultConfig("-q"),
    // is default on CLI, so flag has no value
    TcpKeepAlive(" "),
    CaPath("--capath"),
    SpnegoAuth("--negotiate"),
    Kerberos("--krb"),
    Progress("--progress-bar"),
}

#[cfg(test)]
mod tests {
    use std::ops::DerefMut;

    use super::*;
    use mockito::ServerGuard;
    use serde_json::json;

    fn setup(method: &str) -> ServerGuard {
        let mut server = mockito::Server::new();
        // Start a mock server
        let _ = server
            .mock(method, "/api/resource")
            .with_status(200)
            .with_body("Mocked Response")
            .create();
        server
    }

    #[test]
    fn test_new_curl() {
        let curl = Curl::new();
        assert_eq!(curl.cmd, "curl");
        assert_eq!(curl.opts.len(), 0);
        assert_eq!(curl.resp, None);
    }

    #[test]
    fn test_build_command_str() {
        let url = "https://example.com".to_string();
        let mut curl = Curl::new();
        curl.set_get_method();
        curl.set_verbose(true);
        curl.set_url(&url);
        curl.build_command_str();
        assert_eq!(curl.cmd, format!("curl -X GET {} -v", url));
        assert_eq!(curl.opts.len(), 1);
        assert_eq!(curl.resp, None);
    }

    #[test]
    fn test_set_method() {
        // test POST method
        let mut curl = Curl::new();
        curl.set_post_method();
        curl.build_command_str();
        assert_eq!(curl.cmd, "curl -X POST");

        // Test setting method to GET
        let mut curl_get = Curl::new();
        curl_get.set_get_method();
        curl_get.build_command_str();
        assert_eq!(curl_get.cmd, "curl -X GET");
    }

    #[test]
    fn test_set_url() {
        let mut curl = Curl::new();
        let url = "https://example.com";
        curl.set_url(url);
        curl.set_get_method();
        curl.build_command_str();
        assert_eq!(curl.url, url);
        // get is default method
        assert_eq!(curl.cmd, format!("curl -X GET {}", url));
    }

    #[test]
    fn test_set_response() {
        let mut curl = Curl::new();
        let response = "This is a response";
        curl.set_response(response);
        assert_eq!(curl.resp, Some(String::from(response)));
    }

    #[test]
    fn test_write_output() {
        let mut curl = Curl::new();
        let response = "This is a response";
        curl.set_response(response);
        curl.set_outfile("output.txt");
        curl.write_output().unwrap();
        let _ = std::fs::remove_file("output.txt");
    }

    #[test]
    fn test_set_unix_socket() {
        let mut curl = Curl::new();
        curl.set_unix_socket("/var/run/docker.sock");
        assert_eq!(curl.opts.len(), 1);
        assert!(curl.opts.contains(&CurlFlag::UnixSocket(
            CurlFlagType::UnixSocket.get_value(),
            None
        )));
    }

    #[test]
    fn test_parse_from_json() {
        let mut curl = Curl::new();
        let url = "https://google.com";
        curl.set_url(url);
        curl.set_post_method();
        let json_str = serde_json::to_string(&curl).unwrap();
        let new_curl: Curl = serde_json::from_str(&json_str).unwrap();
        assert_eq!(curl.url, new_curl.url);
        assert_eq!(curl.cmd, new_curl.cmd);
        assert_eq!(curl.opts.len(), new_curl.opts.len());
    }
    #[test]
    fn test_deserialize_raw_str() {
        let json = json!(
        {
                "method": "Get",
                "auth": {"Basic": "username:password"},
                "cmd": "curl -X GET https://example.com",
                "headers": [],
                "url": "https://example.com",
                "opts": [],
                "resp": "This is a response",
                "upload_file": "file.txt",
                "outfile": "output.txt",
        }
        );
        let binding = json.to_string();
        let curl: Curl = serde_json::from_str(&binding).unwrap();
        assert_eq!(curl.auth, AuthKind::Basic("username:password".to_string()));
        assert_eq!(curl.cmd, "curl -X GET https://example.com");
        assert_eq!(curl.url, "https://example.com");
        assert_eq!(curl.opts.len(), 0);
    }
    #[test]
    fn test_serde_json() {
        let mut curl = Curl::new();
        let url = "https://google.com";
        curl.set_url(url);
        curl.set_verbose(true);
        curl.set_get_method();
        // serialize it
        let json_str = serde_json::to_string(&curl).unwrap();

        // deserialize it
        let curl2: Curl = serde_json::from_str(&json_str).unwrap();
        assert_eq!(curl2.url, url);
    }

    #[test]
    fn test_parse_from_json_execute() {
        let mut server = setup("GET");
        let mut curl = Curl::new();
        let url = server.deref_mut().url().clone();
        curl.set_url(&url);
        let json_str = serde_json::to_string(&curl).unwrap();
        let mut new_curl: Curl = serde_json::from_str(&json_str).unwrap();
        new_curl.execute(None).unwrap();
        assert_eq!(new_curl.url, url);
        assert!(new_curl.resp.is_some());
    }

    #[test]
    fn test_set_upload_file() {
        let mut curl = Curl::new();
        curl.set_upload_file("file.txt");
        assert_eq!(curl.opts.len(), 1);
        assert_eq!(curl.upload_file, Some("file.txt".to_string()));
        assert!(curl.opts.contains(&CurlFlag::UploadFile(
            CurlFlagType::UploadFile.get_value(),
            Some("file.txt".to_string())
        )));
    }

    #[test]
    fn test_set_outfile() {
        let mut curl = Curl::new();
        let output = "output.txt";
        curl.set_outfile(output);
        assert_eq!(curl.opts.len(), 1);
        assert_eq!(curl.outfile, Some(output.to_string()));
    }

    #[test]
    fn test_opts_len() {
        let mut curl = Curl::new();
        curl.set_verbose(true);
        assert_eq!(curl.opts.len(), 1);
    }

    #[test]
    fn test_remove_flag() {
        let mut curl = Curl::new();
        let flag = CurlFlag::Verbose(CurlFlagType::Verbose.get_value(), None);
        curl.add_flag(flag.clone());
        assert_eq!(curl.opts.len(), 1);
        curl.remove_flag(&flag);
        assert_eq!(curl.opts.len(), 0);
        assert!(!curl.opts.contains(&flag));
    }

    #[test]
    fn test_add_headers() {
        let mut curl = Curl::new();
        let headers = String::from("Header2: Value2");
        curl.add_headers(headers.clone());
        assert_eq!(curl.opts.len(), 0);
        assert!(curl.headers.is_some());
        assert!(curl.headers.as_ref().unwrap().contains(&headers));
    }

    #[test]
    fn test_execute() {
        let mut server = setup("GET");

        let mut curl = Curl::new();
        curl.set_url(server.url().as_str());
        curl.set_get_method();
        assert!(curl.execute(None).is_ok());
        assert_eq!(curl.url, server.deref_mut().url());
        assert!(curl.resp.is_some());
    }

    #[test]
    fn test_show_headers() {
        let mut curl = Curl::new();
        curl.show_headers("headers.txt");
        assert_eq!(curl.opts.len(), 1);
        assert!(curl.opts.contains(&CurlFlag::DumpHeaders(
            CurlFlagType::DumpHeaders.get_value(),
            Some("headers.txt".to_string())
        )));
    }

    #[test]
    fn test_set_verbose() {
        let mut curl = Curl::new();
        curl.set_verbose(true);
        assert_eq!(curl.opts.len(), 1);
        assert!(curl
            .opts
            .contains(&CurlFlag::Verbose(CurlFlagType::Verbose.get_value(), None)));
    }

    #[test]
    fn test_set_any_auth() {
        let mut curl = Curl::new();
        curl.set_any_auth();
        assert_eq!(curl.opts.len(), 1);
        assert!(curl
            .opts
            .contains(&CurlFlag::AnyAuth(CurlFlagType::AnyAuth.get_value(), None)));
    }

    #[test]
    fn test_set_basic_auth() {
        let mut curl = Curl::new();
        let usr_pwd = "username:password";
        curl.set_basic_auth(usr_pwd.to_string());
        assert_eq!(curl.opts.len(), 1);
        assert!(curl.opts.contains(&CurlFlag::Basic(
            CurlFlagType::Basic.get_value(),
            Some(usr_pwd.to_string())
        )));
    }

    #[test]
    fn test_set_digest_auth() {
        let mut curl = Curl::new();
        curl.set_digest_auth("username:pwd");
        assert_eq!(curl.opts.len(), 1);
        assert!(curl.opts.contains(&CurlFlag::Digest(
            CurlFlagType::Digest.get_value(),
            Some(String::from("username:pwd"))
        )));
    }

    #[test]
    fn test_set_aws_sigv4_auth() {
        let mut curl = Curl::new();
        curl.set_aws_sigv4_auth("user:password".to_string());
        curl.build_command_str();
        assert_eq!(curl.opts.len(), 1);
        assert_eq!(curl.auth, AuthKind::AwsSigv4("user:password".to_string()));
        assert_eq!(curl.cmd, "curl  --aws-sigv4 user:password");
        assert!(curl.opts.contains(&CurlFlag::AwsSigv4(
            CurlFlagType::AwsSigv4.get_value(),
            Some(String::from("user:password"))
        )));
    }

    #[test]
    fn test_set_spnego_auth() {
        let mut curl = Curl::new();
        curl.set_spnego_auth("username:pwd".to_string());
        curl.build_command_str();
        assert_eq!(curl.opts.len(), 1);
        assert_eq!(curl.auth, AuthKind::Spnego("username:pwd".to_string()));
        assert!(curl.opts.contains(&CurlFlag::SpnegoAuth(
            CurlFlagType::SpnegoAuth.get_value(),
            Some(String::from("username:pwd"))
        )));
    }

    #[test]
    fn test_set_get_method() {
        let mut curl = Curl::new();
        let mut server = setup("GET");
        curl.set_get_method();
        let url = server.deref_mut().url();
        curl.set_url(&url);
        curl.build_command_str();
        assert_eq!(curl.cmd, format!("curl -X GET {}", url));
        curl.execute(None).unwrap();
        assert!(curl.resp.is_some());
    }

    #[test]
    fn test_set_post_method() {
        let mut server = setup("POST");
        let url = server.deref_mut().url();
        let mut curl = Curl::new();
        curl.set_post_method();
        curl.set_url(&url);
        curl.build_command_str();
        assert_eq!(curl.cmd, format!("curl -X POST {}", url));
        curl.execute(None).unwrap();
        assert!(curl.resp.is_some());
    }

    #[test]
    fn test_set_put_method() {
        let mut server = setup("PUT");

        let url = server.deref_mut().url();
        let mut curl = Curl::new();
        curl.set_put_method();
        curl.set_url(&url);
        curl.build_command_str();
        assert_eq!(curl.cmd, format!("curl -X PUT {}", url));
        curl.execute(None).unwrap();
        assert!(curl.resp.is_some());
    }

    #[test]
    fn test_set_patch_method() {
        let mut server = setup("PATCH");

        let url = server.deref_mut().url();

        let mut curl = Curl::new();
        curl.set_patch_method();
        curl.set_url(&url);
        curl.build_command_str();
        assert_eq!(curl.cmd, format!("curl -X PATCH {}", url));
        curl.execute(None).unwrap();
        assert!(curl.resp.is_some());
    }

    #[test]
    fn test_set_delete_method() {
        let mut server = setup("DELETE");

        let url = server.deref_mut().url();

        let mut curl = Curl::new();
        curl.set_delete_method();
        curl.set_url(&url);
        curl.build_command_str();
        assert_eq!(curl.cmd, format!("curl -X DELETE {}", url));
        curl.execute(None).unwrap();
        assert!(curl.resp.is_some());
    }
}
