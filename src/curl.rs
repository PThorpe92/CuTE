use crate::Request;
use std::io::Read;
use std::process::{Command, Stdio};

#[derive(Debug, Clone, PartialEq)]
pub struct Curl {
    pub cmd: &'static str,          // The final command string we will run
    pub opts: Vec<Flag>,            // The opts we will build incrementally and store
    pub req: Request,               // The final request we will make
    pub resp: Option<&'static str>, // The response we get back from the command if not sent to file
}

impl Curl {
    pub fn default(url: &str) -> Self {
        Self {
            cmd: "curl",
            opts: Vec::new(),
            req: Request::default(url),
            resp: None,
        }
    }
    pub fn add_flag(&mut self, flag: CurlFlag, value: Option<String>) {
        match flag {
            CurlFlag::Verbose(_) => self.opts.push(Flag {
                flag: CurlFlag::Verbose(String::from("-v")),
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
                flag: CurlFlag::Output(String::from("-o")),
                value: Some(String::from(value.expect("Output file not provided"))),
            }),
            CurlFlag::Trace(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Trace),
                value: Some(String::from(
                    value.expect("File to write trace info to not provided"),
                )),
            }),
            CurlFlag::DataUrlEncode(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::DataUrlEncode),
                value: Some(String::from(
                    value.expect("Data to url encode not provided"),
                )),
            }),
            CurlFlag::DumpHeaders(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::DumpHeaders),
                value: Some(String::from(
                    value.expect("File to dump headers to not provided"),
                )),
            }),
            CurlFlag::Referrer(_) => self.opts.push(Flag {
                flag: CurlFlag::Referrer(String::from("-e")),
                value: Some(String::from(value.expect("Referrer not provided"))),
            }),
            CurlFlag::Insecure(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Insecure),
                value: None,
            }),
            CurlFlag::User(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::User),
                value: Some(String::from(value.expect("username:password not provided"))),
            }),
            CurlFlag::Bearer(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Bearer),
                value: Some(String::from(value.expect("Bearer token not provided"))),
            }),
            CurlFlag::Digest(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Digest),
                value: Some(String::from(
                    value.expect("Initial digest request header not provided"),
                )),
            }),
            CurlFlag::Basic(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Basic),
                value: Some(String::from(value.expect("username:password not provided"))),
            }),
            CurlFlag::Socks5(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Socks5),
                value: Some(String::from(value.expect("Socks5 info not provided"))),
            }),
            CurlFlag::CaCert(_) => self.opts.push(Flag {
                flag: CurlFlag::CaCert(String::from("--cacert")),
                value: Some(String::from(value.expect("Certificate not provided"))),
            }),
            CurlFlag::CaNative(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::CaNative),
                value: None,
            }),
            CurlFlag::File(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::File),
                value: Some(String::from(value.expect("File not provided"))),
            }),
            CurlFlag::FtpAccount(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::FtpAccount),
                value: Some(String::from(value.expect("FTP account not provided"))),
            }),
            CurlFlag::FtpSsl(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::FtpSsl),
                value: None,
            }),
            CurlFlag::CaPath(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::CaPath),
                value: Some(String::from(
                    value.expect("Directory for CaPath not provided"),
                )),
            }),
            CurlFlag::ProxyTunnel(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::ProxyTunnel),
                value: Some(String::from(value.expect("Proxy tunnel info not provided"))),
            }),
            CurlFlag::PreventDefaultConfig(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::PreventDefaultConfig),
                value: None,
            }),
            CurlFlag::UnixSocket(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::UnixSocket),
                value: Some(String::from(value.expect("Socket info not provided"))),
            }),
            CurlFlag::UploadFile(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::UploadFile),
                value: Some(String::from(value.expect("Upload file value not provided"))),
            }),
            CurlFlag::Proxy(_) => self.opts.push(Flag {
                flag: CurlFlag::new(None, CurlFlagType::Proxy),
                value: Some(String::from(value.expect("Proxy value not provided"))),
            }),
        }
        self.opts.push(Flag { flag, value });
    }

    pub fn execute_command(&self) -> Result<&str, std::io::Error> {
        let mut output = Command::new("curl");

        // Add the curl flags to the command
        for flag in &self.opts {
            output.arg(flag.flag.get_value());
            if let Some(argument) = &flag.value {
                output.arg(argument);
            }
        }

        output.arg(&self.req.url).stdout(Stdio::piped());

        // Spawn the command and capture its output
        let child = output.spawn()?;

        // Wait for the command to complete
        let status = child.wait()?;

        if status.success() {
            // If the command was successful, read and return the output
            let mut output_str = String::new();
            if let Some(mut stdout) = child.stdout {
                stdout.read_to_string(&mut output_str)?;
            }
            Ok(output_str.as_str())
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
#[derive(Debug, Clone, PartialEq)]
pub struct Flag {
    pub flag: CurlFlag,
    pub value: Option<String>,
}

#[macro_export]
macro_rules! define_curl_flags {
    (
        $( $variant:ident($value:expr), )*
    ) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum CurlFlag {
            $( $variant(String), )*
        }

        impl CurlFlag {
            pub fn new(value: Option<String>, flag_type: CurlFlagType) -> Self {
                match flag_type {
                    $( CurlFlagType::$variant => CurlFlag::$variant(value.unwrap_or($value.to_string())), )*
                }
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        enum CurlFlagType {
            $( $variant, )*
        }
    };
}
impl CurlFlag {
    pub fn get_value(&self) -> String {
        match *self {
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
        }
    }
}
// Define the CurlFlag enum using the macro.
define_curl_flags! {
    Verbose("-v"),
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
