use std::io::Read;
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub struct Curl<'a> {
    pub cmd: &'a str,          // The final command string we will run
    url: &'a str,              // The url we will send the request to
    pub opts: Vec<Flag<'a>>,   // The opts we will build incrementally and store
    pub resp: Option<&'a str>, // The response we get back from the command if not sent to file
}
// No need to have a request field, we can just build the command incrementally
impl<'a> Curl<'a> {
    pub fn new() -> Self {
        Self {
            cmd: "curl",
            url: "",
            opts: Vec::new(),
            resp: None,
        }
    }

    pub fn default(url: &'a str) -> Self {
        let mut self_ = Self {
            cmd: "curl",
            url: "",
            opts: Vec::new(),
            resp: None,
        };
        self_.add_flag(CurlFlag::new(None, CurlFlagType::Method), Some("GET"));
        self_.add_url(url);
        self_
    }

    pub fn set_method(&mut self, method: &'a str) {
        self.add_flag(CurlFlag::new(None, CurlFlagType::Method), Some(method));
    }

    pub fn add_url(&mut self, url: &'a str) {
        self.url = url;
    }

    pub fn add_flag(&mut self, flag: CurlFlag<'a>, value: Option<&'a str>) {
        match flag {
            CurlFlag::Method(_) => {
                if let Some(val) = value.clone() {
                    self.opts.push(Flag {
                        flag: CurlFlag::new(None, CurlFlagType::Method),
                        value: Some(val).clone(),
                    });
                }
            }
            CurlFlag::Verbose(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Verbose),
                value: None,
            }),
            CurlFlag::AnyAuth(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::AnyAuth),
                value: None,
            }),
            CurlFlag::Ntlm(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Ntlm),
                value: None,
            }),
            CurlFlag::Output(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Output),
                value: Some(value.clone().expect("Output file not provided")),
            }),
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
        }
        self.opts.push(Flag { flag, value });
    }

    pub fn execute_command(&self) -> Result<String, std::io::Error> {
        let mut output = Command::new("curl");

        // This takes each one of our added flags / args and creates the command
        for flag in &self.opts {
            output.arg(flag.flag.get_value());
            if let Some(argument) = &flag.value {
                output.arg(argument);
            }
        }

        // Spawn the command and capture its output
        let mut child = output.spawn()?;

        // Wait for the command to complete
        let status = child.wait()?;

        if status.success() {
            // If the command was successful, read and return the output
            let mut output_str = String::new();
            if let Some(mut stdout) = child.stdout {
                stdout.read_to_string(&mut output_str)?;
                Ok(output_str)
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to capture standard output",
                ));
            }
        } else {
            // Handle the case when the command fails
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Command failed with {:?}", status),
            ))
        }
    }
}

// curl.opts  =  Vec<Flag>  =  vec!["--cert-type", "PEM"] so flag / argument
// but we dont want to have to provide/remember the "-X"(flag) so we store it in the enum
// We may have "--verbose" which is a flag with no value
// But each enum variant has the default flag stored as a static string, so we can use that
// to build the command incrementally by just providing the argument value when we create the flag.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Flag<'a> {
    pub flag: CurlFlag<'a>,
    pub value: Option<&'a str>,
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
        match self.clone() {
            CurlFlag::Verbose(val) => val,
            CurlFlag::Output(val) => val,
            CurlFlag::User(val) => val,
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
            CurlFlag::Referrer(val) => val,
            CurlFlag::Insecure(val) => val,
            CurlFlag::PreventDefaultConfig(val) => val,
            CurlFlag::CaCert(val) => val,
            CurlFlag::CaNative(val) => val,
            CurlFlag::CaPath(val) => val,
            CurlFlag::Method(val) => val,
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
    Proxy("-x"),
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
}
