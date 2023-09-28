
use std::io::Write;

use curl::easy::{Auth, Easy, Handler, List};

#[derive(Debug)]
pub struct Curl<'a> {
      // The libcurl interface for our command/request
    curl: Easy,
    // The auth type we will use
    auth: AuthType,
    // The final command string we will run
    cmd: String,
    // The url we will send the request to
    url: String,
    // The opts we will build incrementally and store
    opts: Vec<Flag<'a>>,
    // The response we get back from the command if not sent to file
    resp: Option<String>,
  // Filepath of file to be uploaded 
    upload_file: Option<String>,
  // Filepath of the response output file or download
    outfile: Option<String>,
}
#[derive(Debug, Copy, Clone, PartialEq)]
enum AuthType {
    None,
    Ntlm,
    Basic,
    Digest,
    AwsSigv4,
    Spegno,
    NtlmWb,
    DigestIe,
}

impl<'a> Curl<'a> {
    pub fn new() -> Self {
        Self {
            curl: Easy::new(),
            auth: AuthType::None,
            cmd: String::from("curl "),
            url: String::new(),
            opts: Vec::new(),
            resp: None,
            upload_file: None,
            outfile: None,
        }
    }

    pub fn default(url: String) -> Self {
        let mut default = Self {
            curl: Easy::new(),
            auth: AuthType::None,
            cmd: String::from("curl "),
            url: String::new(),
            opts: Vec::new(),
            resp: None,
            upload_file: None,
            outfile: None,
        };
        default.add_flag(
            CurlFlag::new(None, CurlFlagType::Method),
            Some(String::from("GET")),
        );
        default.set_url(url.clone().to_string());
        default
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url.clone();
        self.curl.url(url.as_str()).unwrap();
    }

    pub fn set_response(&mut self, response: String) {
        self.resp = Some(response.clone());
    }

    pub fn write_output(&mut self) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::create(self.outfile.clone().expect("./output.txt"))?;
        let mut writer = std::io::BufWriter::new(&mut file);
        let _ = writer.write_all(self.resp.clone().unwrap().as_bytes());
        Ok(())
    }

    pub fn remove_flag(&mut self, flag: Flag<'a>) {
        self.opts.retain(|x| *x != flag);
    }

    pub fn add_headers(&mut self, headers: Vec<String>) {
        let mut list = List::new();
        let _ = headers.iter().map(|h| list.append(h.as_str()).unwrap());
        self.curl.http_headers(list).unwrap();
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.add_flag(CurlFlag::new(None, CurlFlagType::Verbose), None);
        self.curl.verbose(verbose).unwrap_or_default();
    }

    pub fn set_any_auth(&mut self) {
        self.add_flag(CurlFlag::new(None, CurlFlagType::AnyAuth), None);
        let _ = self.curl.http_auth(&Auth::new());
    }

    pub fn set_basic_auth(&mut self, login: String) {
        self.add_flag(CurlFlag::new(None, CurlFlagType::Basic), Some(login));
        self.auth = AuthType::Basic;
    }

    pub fn set_digest_auth(&mut self, info: &str) {
        self.add_flag(
            CurlFlag::new(None, CurlFlagType::Digest),
            Some(info.to_string()),
        );
        self.auth = AuthType::Digest;
    }

    pub fn set_aws_sigv4_auth(&mut self, on: bool) {
        if on {
            self.add_flag(CurlFlag::new(None, CurlFlagType::Digest), None);
            self.auth = AuthType::AwsSigv4;
        } else {
            self.remove_flag(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Digest),
                value: None,
            });
            self.auth = AuthType::None;
        }
    }

    pub fn set_digest_ie_auth(&mut self, on: bool) {
        if on {
            self.add_flag(CurlFlag::new(None, CurlFlagType::Digest), None);
            self.auth = AuthType::DigestIe;
        } else {
            self.remove_flag(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Digest),
                value: None,
            });
            self.auth = AuthType::DigestIe;
        }
    }

    pub fn set_spegno_auth(&mut self, on: bool) {
        if on {
            self.add_flag(CurlFlag::new(None, CurlFlagType::SpegnoAuth), None);
            self.auth = AuthType::Spegno;
        } else {
            self.remove_flag(Flag {
                flag: CurlFlag::new(None, CurlFlagType::SpegnoAuth),
                value: None,
            });
            self.auth = AuthType::Spegno;
        }
    }

    pub fn set_get_method(&mut self, method: bool) {
        if method {
            self.add_flag(
                CurlFlag::new(None, CurlFlagType::Method),
                Some(String::from("GET")),
            );
        } else {
            self.remove_flag(Flag {
                flag: CurlFlag::new(Some("GET"), CurlFlagType::Method),
                value: Some(String::from("GET")),
            });
        }
        self.curl.get(method).unwrap();
    }

    pub fn set_post_method(&mut self, method: bool) {
        if method {
            self.add_flag(
                CurlFlag::new(None, CurlFlagType::Method),
                Some(String::from("POST")),
            );
        } else {
            self.remove_flag(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Method),
                value: Some(String::from("POST")),
            });
        }
        self.curl.post(method).unwrap();
    }

    pub fn set_put_method(&mut self, method: bool) {
        if method {
            self.add_flag(
                CurlFlag::new(None, CurlFlagType::Method),
                Some(String::from("PUT")),
            );
        } else {
            self.remove_flag(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Method),
                value: Some(String::from("PUT")),
            });
        }
        self.curl.put(method).unwrap();
    }

    pub fn set_patch_method(&mut self, method: bool) {
        if method {
            self.add_flag(
                CurlFlag::new(Some("PATCH"), CurlFlagType::Method),
                Some(String::from("PATCH")),
            );
            self.curl.custom_request("PATCH").unwrap();
        } else {
            self.remove_flag(Flag {
                flag: CurlFlag::new(Some("PATCH"), CurlFlagType::Method),
                value: Some(String::from("PATCH")),
            });
            self.curl.custom_request("").unwrap();
        }
    }

    pub fn set_delete_method(&mut self, method: bool) {
        if method {
            self.add_flag(
                CurlFlag::new(None, CurlFlagType::Method),
                Some(String::from("DELETE")),
            );
            self.curl.custom_request("DELETE").unwrap();
        } else {
            self.remove_flag(Flag {
                flag: CurlFlag::new(Some("DELETE"), CurlFlagType::Method),
                value: Some(String::from("DELETE")),
            });
            self.curl.custom_request("").unwrap();
        }
    }

    pub fn set_ntlm_auth(&mut self, on: bool) {
        if on {
            self.add_flag(CurlFlag::new(None, CurlFlagType::Ntlm), None);
        } else {
            self.remove_flag(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Ntlm),
                value: None,
            });
        }
        self.auth = AuthType::Ntlm;
    }

    pub fn set_ntlm_wb_auth(&mut self, on: bool) {
        if on {
            self.add_flag(CurlFlag::new(None, CurlFlagType::Ntlm), None);
        } else {
            self.remove_flag(Flag {
                flag: CurlFlag::new(None, CurlFlagType::NtlmWb),
                value: None,
            });
        }
        self.auth = AuthType::NtlmWb;
    }

    pub fn set_progress(&mut self, on: bool) {
        if on {
            self.add_flag(CurlFlag::new(None, CurlFlagType::Progress), None);
        } else {
            self.remove_flag(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Progress),
                value: None,
            });
        }
        self.curl.progress(on).unwrap();
    }

    pub fn set_output(&mut self, output: String) {
        self.add_flag(
            CurlFlag::new(None, CurlFlagType::Output),
            Some(output.clone()),
        );
        self.outfile = Some(output.clone());
    }
    // pub fn set_output(&mut self, output: String) {
    //     self.add_flag(
    //         CurlFlag::new(None, CurlFlagType::Output),
    //         Some(output.clone()),
    //     );
    //     self.curl
    //         .write_function(move |data| {
    //             self.resp = Some(String::from_utf8_lossy(data).to_string());
    //             Ok(data.len())
    //         })
    //         .unwrap();
    // }

    pub fn set_unix_socket(&mut self, socket: &str) {
        self.add_flag(
            CurlFlag::new(None, CurlFlagType::UnixSocket),
            Some(socket.to_string()),
        );
        self.curl.unix_socket(socket.clone()).unwrap();
    }

    pub fn show_headers(&mut self, file: &str) {
        self.add_flag(
            CurlFlag::new(None, CurlFlagType::DumpHeaders),
            Some(file.to_string()),
        );
        self.curl.show_header(true).unwrap();
    }

    pub fn set_upload_file(&mut self, file: &str) {
        self.add_flag(
            CurlFlag::new(None, CurlFlagType::UploadFile),
            Some(file.to_string()),
        );
        self.upload_file = Some(file.to_string());
    }

    pub fn build_command_str(&mut self) {
        for flag in &self.opts {
            self.cmd.push_str(flag.flag.get_value());
            self.cmd.push(' ');
            if let Some(arg) = &flag.value {
                self.cmd.push_str(arg.as_str());
                self.cmd.push(' ');
            }
        }
        self.cmd.push_str(self.url.as_str());
        self.cmd = self.cmd.trim().to_string();
    }

    pub fn add_flag(&mut self, flag: CurlFlag<'a>, value: Option<String>) {
        match flag {
            CurlFlag::Method(_) => {
                if let Some(val) = value {
                    self.opts.push(Flag {
                        flag: CurlFlag::new(None, CurlFlagType::Method),
                        value: Some(val),
                    });
                }
            }
            CurlFlag::Verbose(_) => {
                self.opts.push(Flag {
                    flag: CurlFlag::new(None, CurlFlagType::Verbose),
                    value: None,
                });
            }
            CurlFlag::AnyAuth(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::AnyAuth),
                value: None,
            }),
            CurlFlag::Ntlm(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Ntlm),
                value: None,
            }),
            CurlFlag::Output(_) => {
                self.opts.push(Flag {
                    flag: CurlFlag::new(None, CurlFlagType::Output),
                    value: Some(value.clone().expect("Output file not provided")),
                });
                self.outfile = Some(value.clone().unwrap_or(String::from("output.txt")));
            }
            CurlFlag::Trace(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Trace),
                value: Some(
                    value
                        .clone()
                        .expect("File to write trace info to not provided"),
                ),
            }),
            CurlFlag::DataUrlEncode(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::DataUrlEncode),
                value: Some(value.clone().expect("Data to url encode not provided")),
            }),
            CurlFlag::DumpHeaders(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::DumpHeaders),
                value: Some(value.clone().expect("File to dump headers to not provided")),
            }),
            CurlFlag::Referrer(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Referrer),
                value: Some(value.clone().expect("Referrer not provided")),
            }),
            CurlFlag::Insecure(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Insecure),
                value: None,
            }),
            CurlFlag::User(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::User),
                value: Some(value.clone().expect("username:password not provided")),
            }),
            CurlFlag::Bearer(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Bearer),
                value: Some(value.clone().expect("Bearer token not provided")),
            }),
            CurlFlag::Digest(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Digest),
                value: Some(
                    value
                        .clone()
                        .expect("Initial digest request header not provided"),
                ),
            }),
            CurlFlag::Basic(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Basic),
                value: Some(value.clone().expect("username:password not provided")),
            }),
            CurlFlag::Socks5(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Socks5),
                value: Some(value.clone().expect("Socks5 info not provided")),
            }),
            CurlFlag::CaCert(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::CaCert),
                value: Some(value.clone().expect("Certificate not provided")),
            }),
            CurlFlag::CaNative(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::CaNative),
                value: None,
            }),
            CurlFlag::File(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::File),
                value: Some(value.clone().expect("File not provided")),
            }),
            CurlFlag::FtpAccount(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::FtpAccount),
                value: Some(value.clone().expect("FTP account not provided")),
            }),
            CurlFlag::FtpSsl(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::FtpSsl),
                value: None,
            }),
            CurlFlag::CaPath(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::CaPath),
                value: Some(value.clone().expect("Directory for CaPath not provided")),
            }),
            CurlFlag::ProxyTunnel(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::ProxyTunnel),
                value: Some(value.clone().expect("Proxy tunnel info not provided")),
            }),
            CurlFlag::PreventDefaultConfig(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::PreventDefaultConfig),
                value: None,
            }),
            CurlFlag::UnixSocket(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::UnixSocket),
                value: Some(value.clone().expect("Socket info not provided")),
            }),
            CurlFlag::UploadFile(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::UploadFile),
                value: Some(value.clone().expect("Upload file value not provided")),
            }),
            CurlFlag::Proxy(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Proxy),
                value: Some(value.clone().expect("Proxy value not provided")),
            }),
            CurlFlag::SpegnoAuth(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::SpegnoAuth),
                value: None,
            }),
            CurlFlag::Progress(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Progress),
                value: None,
            }),
            CurlFlag::AwsSigv4(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::AwsSigv4),
                value: None,
            }),
            CurlFlag::NtlmWb(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::NtlmWb),
                value: None,
            }),
        }
    }

    pub fn execute(&mut self) -> Result<String, curl::Error> {
        let mut data = Vec::new();
        let auth = match self.auth {
            AuthType::Basic => self.curl.http_auth(&Auth::new().basic(true)),
            AuthType::Digest => self.curl.http_auth(&Auth::new().digest(true)),
            AuthType::Ntlm => self.curl.http_auth(&Auth::new().ntlm(true)),
            AuthType::NtlmWb => self.curl.http_auth(&Auth::new().ntlm_wb(true)),
            AuthType::Spegno => self.curl.http_auth(&Auth::new().gssnegotiate(true)),
            AuthType::DigestIe => self.curl.http_auth(&Auth::new().digest_ie(true)),
            AuthType::AwsSigv4 => self.curl.http_auth(&Auth::new().aws_sigv4(true)),
            _ => Ok(()),
        };
        match auth {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        {
            let mut transfer = self.curl.transfer();
            transfer
                .write_function(|new_data| {
                    data.extend_from_slice(new_data);
                    Ok(new_data.len())
                })
                .unwrap();
            transfer.perform().unwrap();
        }
        return Ok(String::from_utf8(data).unwrap());
    }
}

// curl.opts  =  Vec<Flag>  =  vec!["--cert-type", "PEM"] so flag / argument
// but we dont want to have to provide/remember the "-X"(flag) so we store it in the enum
// We may have "--verbose" which is a flag with no value
// But each enum variant has the default flag stored as a static string, so we can use that
// to build the command incrementally by just providing the argument value when we create the flag.
#[derive(Debug, Clone, PartialEq)]
pub struct Flag<'a> {
    pub flag: CurlFlag<'a>,
    pub value: Option<String>,
}

#[macro_export]
macro_rules! define_curl_flags {
    (
        $( $variant:ident($value:expr), )*
    ) => {
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub enum CurlFlag<'a> {
            $( $variant(&'a str), )*
        }

        impl<'a> CurlFlag<'a> {
            pub fn new(value: Option<&'a str>, flag_type: CurlFlagType) -> Self {
                match flag_type {
                    $( CurlFlagType::$variant => CurlFlag::$variant(value.unwrap_or($value)), )*
                }
            }
        }

        #[derive(Debug, Copy, Clone, PartialEq)]
        pub enum CurlFlagType {
            $( $variant, )*
        }
    };
}
impl CurlFlag<'_> {
    pub fn get_value(&self) -> &str {
        match *self {
            CurlFlag::Verbose(val) => val,
            CurlFlag::Output(val) => val,
            CurlFlag::User(val) => val,
            CurlFlag::NtlmWb(val) => val,
            CurlFlag::AwsSigv4(val) => val,
            CurlFlag::Bearer(val) => val,
            CurlFlag::Digest(val) => val,
            CurlFlag::Basic(val) => val,
            CurlFlag::AnyAuth(val) => val,
            CurlFlag::UnixSocket(val) => val,
            CurlFlag::UploadFile(val) => val,
            CurlFlag::Ntlm(val) => val,
            CurlFlag::Proxy(val) => val,
            CurlFlag::ProxyTunnel(val) => val,
            CurlFlag::Socks5(val) => val,
            CurlFlag::File(val) => val,
            CurlFlag::FtpAccount(val) => val,
            CurlFlag::FtpSsl(val) => val,
            CurlFlag::Trace(val) => val,
            CurlFlag::DataUrlEncode(val) => val,
            CurlFlag::DumpHeaders(val) => val,
            CurlFlag::Progress(val) => val,
            CurlFlag::Referrer(val) => val,
            CurlFlag::Insecure(val) => val,
            CurlFlag::PreventDefaultConfig(val) => val,
            CurlFlag::CaCert(val) => val,
            CurlFlag::CaNative(val) => val,
            CurlFlag::CaPath(val) => val,
            CurlFlag::Method(val) => val,
            CurlFlag::SpegnoAuth(val) => val,
        }
    }
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
    SpegnoAuth("--negotiate"),
    Progress("--progress-bar"),
}

#[cfg(test)]
mod tests {
    use super::*;

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
        curl.set_get_method(true);
        curl.set_verbose(true);
        curl.set_url(url.clone());
        curl.build_command_str();
        assert_eq!(curl.cmd, format!("curl -X GET -v {}", url));
        assert_eq!(curl.opts.len(), 2);
        assert_eq!(curl.resp, None);
    }

    #[test]
    fn test_set_method() {
        // test POST method
        let mut curl = Curl::new();
        curl.set_post_method(true);
        curl.build_command_str();
        assert_eq!(curl.opts.len(), 1);
        assert_eq!(curl.cmd, "curl -X POST");

        // Test setting method to GET
        let mut curl_get = Curl::new();
        curl_get.set_get_method(true);
        curl_get.build_command_str();
        assert_eq!(curl_get.opts.len(), 1);
        assert_eq!(curl_get.cmd, "curl -X GET");
    }

    #[test]
    fn test_set_url() {
        let mut curl = Curl::new();
        let url = "https://example.com".to_string();
        curl.set_url(url.clone());
        curl.build_command_str();
        assert_eq!(curl.url, url);
        assert_eq!(curl.cmd, format!("curl {}", url));
    }

    #[test]
    fn test_set_response() {
        let mut curl = Curl::new();
        let response = "This is a response".to_string();
        curl.set_response(response.clone());
        assert_eq!(curl.resp, Some(response));
    }

    #[test]
    fn test_write_output() {
        let mut curl = Curl::new();
        let response = "This is a response".to_string();
        curl.set_response(response.clone());
        curl.set_output("output.txt".to_string());
        curl.write_output().unwrap();
    }

    #[test]
    fn test_set_unix_socket() {
        let mut curl = Curl::new();
        curl.set_unix_socket("/var/run/docker.sock");
        assert_eq!(curl.opts.len(), 1);
        assert!(curl.opts.contains(&Flag {
            flag: CurlFlag::new(None, CurlFlagType::UnixSocket),
            value: Some(String::from("/var/run/docker.sock")),
        }));
    }

    #[test]
    fn test_set_upload_file() {
        let mut curl = Curl::new();
        curl.set_upload_file("file.txt");
        assert_eq!(curl.opts.len(), 1);
        assert_eq!(curl.upload_file, Some("file.txt".to_string()));
        assert!(curl.opts.contains(&Flag {
            flag: CurlFlag::new(None, CurlFlagType::UploadFile),
            value: Some(String::from("file.txt")),
        }));
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
        let flag = CurlFlag::new(None, CurlFlagType::Verbose);
        curl.add_flag(flag.clone(), None);
        assert_eq!(curl.opts.len(), 1);
        curl.remove_flag(Flag {
            flag: flag.clone(),
            value: None,
        });
        assert_eq!(curl.opts.len(), 0);
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
        let mut curl = Curl::new();
        let url = String::from("ifconfig.me");
        curl.set_url(url.clone());
        assert!(curl.execute().is_ok());
        assert_eq!(curl.url, url);
    }

    #[test]
    fn test_show_headers() {
        let mut curl = Curl::new();
        curl.show_headers("headers.txt");
        assert_eq!(curl.opts.len(), 1);
        assert!(curl.opts.contains(&Flag {
            flag: CurlFlag::new(None, CurlFlagType::DumpHeaders),
            value: Some(String::from("headers.txt")),
        }));
    }

    #[test]
    fn test_set_verbose() {
        let mut curl = Curl::new();
        curl.set_verbose(true);
        assert_eq!(curl.opts.len(), 1);
    }

    #[test]
    fn test_set_any_auth() {
        let mut curl = Curl::new();
        curl.set_any_auth();
        assert_eq!(curl.opts.len(), 1);
        assert!(curl.opts.contains(&Flag {
            flag: CurlFlag::new(None, CurlFlagType::AnyAuth),
            value: None
        }));
    }

    #[test]
    fn test_set_basic_auth() {
        let mut curl = Curl::new();
        let usr_pwd = "username:password";
        curl.set_basic_auth(usr_pwd.to_string());
        assert_eq!(curl.opts.len(), 1);
        assert!(curl.opts.contains(&Flag {
            flag: CurlFlag::new(None, CurlFlagType::Basic),
            value: Some(usr_pwd.to_string()),
        }));
    }
    #[test]
    fn test_set_digest_auth() {
        let mut curl = Curl::new();
        curl.set_digest_auth("username:pwd");
        assert_eq!(curl.opts.len(), 1);
        assert!(curl.opts.contains(&Flag {
            flag: CurlFlag::new(None, CurlFlagType::Digest),
            value: Some(String::from("username:pwd")),
        }));
    }
}
