use crate::database::{self, db::DB};
use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    io::Write,
};

use curl::easy::{Auth, Easy, List};

#[derive(Debug)]
pub struct Curl<'a> {
    // The libcurl interface for our command/request
    curl: Easy,
    // The auth type we will use
    auth: AuthKind,
    // The final command string we will run
    cmd: String,
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
    // Whether to save the command to DB after execution
    save: bool,
}

#[derive(Debug, Clone, PartialEq)]
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

impl<'a> Default for Curl<'a> {
    fn default() -> Self {
        Self {
            curl: Easy::new(),
            auth: AuthKind::None,
            cmd: String::from("curl "),
            url: String::new(),
            opts: Vec::new(),
            resp: None,
            upload_file: None,
            outfile: None,
            save: false,
        }
    }
}

impl<'a> Curl<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_url(&mut self, url: &str) {
        self.url = String::from(url);
        self.curl.url(url).unwrap();
    }

    pub fn set_response(&mut self, response: &str) {
        self.resp = Some(String::from(response));
    }

    pub fn get_response(&self) -> Option<String> {
        self.resp.clone()
    }

    pub fn write_output(&mut self) -> Result<(), std::io::Error> {
        match self.outfile {
            Some(ref mut outfile) => {
                let mut file = std::fs::File::create(outfile)?;
                let mut writer = std::io::BufWriter::new(&mut file);
                let _ = writer.write_all(self.resp.clone().unwrap().as_bytes());
                Ok(())
            }
            None => Ok(()),
        }
    }

    pub fn remove_flag(&mut self, flag: CurlFlag<'a>) {
        self.opts.retain(|x| *x != flag);
    }

    pub fn add_headers(&mut self, headers: Vec<String>) {
        let mut list = List::new();
        let _ = headers.iter().map(|h| list.append(h.as_str()).unwrap());
        self.curl.http_headers(list).unwrap();
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.add_flag(CurlFlag::Verbose(CurlFlagType::Verbose.get_value(), None));
        self.curl.verbose(verbose).unwrap_or_default();
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
    pub fn will_store_command(&self) -> bool {
        self.save
    }

    pub fn set_get_method(&mut self) {
        self.add_flag(CurlFlag::Method(
            CurlFlagType::Method.get_value(),
            Some(String::from("GET")),
        ));
        self.curl.get(true).unwrap();
    }

    pub fn set_post_method(&mut self) {
        self.add_flag(CurlFlag::Method(
            CurlFlagType::Method.get_value(),
            Some(String::from("POST")),
        ));
        self.curl.post(true).unwrap();
    }

    pub fn set_put_method(&mut self) {
        self.add_flag(CurlFlag::Method(
            CurlFlagType::Method.get_value(),
            Some(String::from("PUT")),
        ));
        self.curl.put(true).unwrap();
    }

    pub fn set_patch_method(&mut self) {
        self.add_flag(CurlFlag::Method(
            CurlFlagType::Method.get_value(),
            Some(String::from("PATCH")),
        ));
        self.curl.custom_request("PATCH").unwrap();
    }

    pub fn set_delete_method(&mut self) {
        self.add_flag(CurlFlag::Method(
            CurlFlagType::Method.get_value(),
            Some(String::from("DELETE")),
        ));
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

    pub fn set_progress(&mut self, on: bool) {
        if on {
            self.add_flag(CurlFlag::Progress(CurlFlagType::Progress.get_value(), None));
        } else {
            self.remove_flag(CurlFlag::Progress(CurlFlagType::Progress.get_value(), None));
        }
        self.curl.progress(on).unwrap();
    }

    pub fn save_command(&mut self, save: bool) {
        self.save = save;
    }

    pub fn set_output(&mut self, output: String) {
        self.add_flag(CurlFlag::Output(CurlFlagType::Output.get_value(), None));
        self.outfile = Some(output.clone());
    }

    pub fn set_unix_socket(&mut self, socket: &str) {
        self.add_flag(CurlFlag::UnixSocket(
            CurlFlagType::UnixSocket.get_value(),
            None,
        ));
        self.curl.unix_socket(socket.clone()).unwrap();
    }

    pub fn set_bearer_auth(&mut self, token: String) {
        self.add_flag(CurlFlag::Bearer(
            CurlFlagType::Bearer.get_value(),
            Some(token.to_string()),
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

    pub fn set_upload_file(&mut self, file: &str) {
        self.add_flag(CurlFlag::UploadFile(
            CurlFlagType::UploadFile.get_value(),
            Some(file.to_string()),
        ));
        self.upload_file = Some(file.to_string());
    }

    pub fn build_command_str(&mut self) {
        for flag in &self.opts {
            self.cmd.push_str(flag.get_value());
            self.cmd.push(' ');
            if let Some(arg) = &flag.get_arg() {
                self.cmd.push_str(arg.as_str());
                self.cmd.push(' ');
            }
        }
        self.cmd.push_str(self.url.as_str());
        self.cmd = self.cmd.trim().to_string();
    }

    pub fn add_flag(&mut self, flag: CurlFlag<'a>) {
        self.opts.push(flag.clone());
    }

    pub fn execute(&mut self, db: &mut Option<Box<DB>>) -> Result<(), curl::Error> {
        if self.save {
            let _ = self.build_command_str();
            db.as_mut().unwrap().add_command(&self.cmd.clone()).unwrap();
        }
        match &self.auth {
            AuthKind::Basic(login) => {
                self.curl
                    .username(login.split(':').next().unwrap())
                    .unwrap();
                self.curl
                    .password(login.split(':').last().unwrap())
                    .unwrap();
                let _ = self.curl.http_auth(Auth::new().basic(true));
            }
            // for some reason, libcurl doesn't support bearer: token, so we have to do it manually
            AuthKind::Bearer(token) => {
                let mut list = List::new();
                let _ = list.append(&format!("Authorization: Bearer {}", token.clone()));
                self.curl.http_headers(list).unwrap();
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
        let data = RefCell::new(Vec::new());
        let mut transfer = self.curl.transfer();
        {
            transfer
                .write_function(|new_data| {
                    let mut data = data.borrow_mut();
                    data.extend_from_slice(new_data);
                    Ok(new_data.len())
                })
                .unwrap();
            transfer.perform().unwrap();
            let res = String::from_utf8(data.take()).unwrap();
            self.resp = Some(res.clone());
            Ok(())
        }
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
        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Copy, Clone, PartialEq)]
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
    Method("-X"),
    Output("-o"),
    User("-u"),
    Bearer("-H"),
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
    Socks5("--socks5"),
    File("-F"),
    FtpAccount("--ftp-account"),
    FtpSsl("--ftp-ssl"),
    Trace("--trace"),
    DataUrlEncode("--data-urlencode"),
    DumpHeaders("--dump-headers"),
    Referrer("-e"),
    Insecure("--insecure"),
    PreventDefaultConfig("-q"),
    CaCert("--cacert"),
    CaNative("--ca-native"),
    CaPath("--capath"),
    SpnegoAuth("--negotiate"),
    Kerberos("--krb"),
    Progress("--progress-bar"),
}

#[cfg(test)]
mod tests {
    use std::ops::DerefMut;

    use mockito::ServerGuard;

    use super::*;

    fn setup(method: &str) -> ServerGuard {
        let mut server = mockito::Server::new();
        // Start a mock server
        let _ = server
            .mock(method, "/api/resource")
            .with_status(200)
            .with_body("Mocked Response")
            .create();
        return ServerGuard::from(server);
    }

    #[test]
    fn test_new_curl() {
        let curl = Curl::new();
        assert_eq!(curl.cmd, "curl ");
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
        assert_eq!(curl.cmd, format!("curl -X GET -v {}", url));
        assert_eq!(curl.opts.len(), 2);
        assert_eq!(curl.resp, None);
    }

    #[test]
    fn test_set_method() {
        // test POST method
        let mut curl = Curl::new();
        curl.set_post_method();
        curl.build_command_str();
        assert_eq!(curl.opts.len(), 1);
        assert_eq!(curl.cmd, "curl -X POST");

        // Test setting method to GET
        let mut curl_get = Curl::new();
        curl_get.set_get_method();
        curl_get.build_command_str();
        assert_eq!(curl_get.opts.len(), 1);
        assert_eq!(curl_get.cmd, "curl -X GET");
    }

    #[test]
    fn test_set_url() {
        let mut curl = Curl::new();
        let url = "https://example.com";
        curl.set_url(url);
        curl.build_command_str();
        assert_eq!(curl.url, url);
        assert_eq!(curl.cmd, format!("curl {}", url));
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
        curl.set_output("output.txt".to_string());
        curl.write_output().unwrap();
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
    fn test_set_output() {
        let mut curl = Curl::new();
        let output = "output.txt".to_string();
        curl.set_output(output.clone());
        assert_eq!(curl.opts.len(), 1);
        assert_eq!(curl.outfile, Some(output.clone()));
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
        curl.remove_flag(flag.clone());
        assert_eq!(curl.opts.len(), 0);
        assert!(!curl.opts.contains(&flag));
    }

    #[test]
    fn test_add_headers() {
        let mut curl = Curl::new();
        let headers = vec![
            String::from("Header1: Value1"),
            String::from("Header2: Value2"),
        ];
        curl.add_headers(headers.clone());
        assert_eq!(curl.opts.len(), 0);
    }

    #[test]
    fn test_execute() {
        let mut server = setup("GET");

        let mut curl = Curl::new();
        curl.set_url(server.url().as_str());
        curl.set_get_method();
        assert!(curl.execute(&mut None).is_ok());
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
        assert_eq!(curl.cmd, "curl --aws-sigv4 user:password");
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
        assert_eq!(curl.opts.len(), 1);
        assert_eq!(curl.cmd, format!("curl -X GET {}", url));
        assert!(curl.opts.contains(&CurlFlag::Method(
            CurlFlagType::Method.get_value(),
            Some(String::from("GET"))
        )));
        curl.execute(&mut None).unwrap();
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
        assert_eq!(curl.opts.len(), 1);
        assert_eq!(curl.cmd, format!("curl -X POST {}", url));
        assert!(curl.opts.contains(&CurlFlag::Method(
            CurlFlagType::Method.get_value(),
            Some(String::from("POST"))
        )));
        curl.execute(&mut None).unwrap();
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
        assert_eq!(curl.opts.len(), 1);
        assert_eq!(curl.cmd, format!("curl -X PUT {}", url));
        assert!(curl.opts.contains(&CurlFlag::Method(
            CurlFlagType::Method.get_value(),
            Some(String::from("PUT"))
        )));
        curl.execute(&mut None).unwrap();
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
        assert_eq!(curl.opts.len(), 1);
        assert_eq!(curl.cmd, format!("curl -X PATCH {}", url));
        assert!(curl.opts.contains(&CurlFlag::Method(
            CurlFlagType::Method.get_value(),
            Some(String::from("PATCH"))
        )));
        curl.execute(&mut None).unwrap();
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
        assert_eq!(curl.opts.len(), 1);
        assert_eq!(curl.cmd, format!("curl -X DELETE {}", url));
        assert!(curl.opts.contains(&CurlFlag::Method(
            CurlFlagType::Method.get_value(),
            Some(String::from("DELETE"))
        )));
        curl.execute(&mut None).unwrap();
        assert!(curl.resp.is_some());
    }
}
