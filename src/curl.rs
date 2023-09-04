use crate::Request;

#[derive(Debug, Clone, PartialEq)]
pub struct Curl {
    pub cmd: &'static str,          // The final command string we will run
    pub opts: Vec<Flag>,            // The opts we will build incrementally and store
    pub req: Request,               // The final request we will make
    pub resp: Option<&'static str>, // The response we get back from the command if not sent to file
}
impl Curl {
    pub fn default() -> Self {
        Self {
            cmd: "curl",
            opts: Vec::new(),
            req: Request::default(),
            resp: None,
        }
    }
}

// So Curl.opts can each have the string value of the flag stored with it's enum,
// and have the value passed to the command line stored in this struct field
#[derive(Debug, Clone, PartialEq)]
pub struct Flag {
    pub flag: CurlFlag,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CurlFlag {
    Verbose(String),
    Output(String),
    User(String),
    Bearer(String),
    Digest(String),
    Basic(String),
    AnyAuth(String),
    UnixSocket(String),
    UploadFile(String),
    Ntlm(String),
    Proxy(String),
    ProxyTunnel(String),
    Socks5(String),
    Dict(String),
    File(String),
    FtpAccount(String),
    FtpSsl(String),
    Trace(String),
    DataUrlEncode(String),
    DumpHeaders(String),
    Referrer(String),
    PreventDefaultConfig(String),
}
impl CurlFlag {
    pub fn new(&self) -> Self {
        match self {
            CurlFlag::AnyAuth(_) => CurlFlag::AnyAuth(String::from("--any-auth")),
            CurlFlag::Verbose(_) => CurlFlag::Verbose(String::from("-v")),
            CurlFlag::Output(_) => CurlFlag::Output(String::from("-o")),
            CurlFlag::User(_) => CurlFlag::User(String::from("-u")),
            CurlFlag::Bearer(_) => CurlFlag::Bearer(String::from("-H")),
            CurlFlag::Digest(_) => CurlFlag::Digest(String::from("--digest")),
            CurlFlag::Basic(_) => CurlFlag::Basic(String::from("-H")),
            CurlFlag::UnixSocket(_) => CurlFlag::UnixSocket(String::from("--unix-socket")),
            CurlFlag::UploadFile(_) => CurlFlag::UploadFile(String::from("--upload-file")),
            CurlFlag::Proxy(_) => CurlFlag::Proxy(String::from("-x")),
            CurlFlag::Dict(_) => CurlFlag::Dict(String::from("-")),
            CurlFlag::File(_) => CurlFlag::File(String::from("-F")),
            CurlFlag::Socks5(_) => CurlFlag::Socks5(String::from("--socks5")),
            CurlFlag::FtpAccount(_) => CurlFlag::FtpAccount(String::from("--ftp-account")),
            CurlFlag::Ntlm(_) => CurlFlag::Ntlm(String::from("--ntlm")),
            CurlFlag::ProxyTunnel(_) => CurlFlag::ProxyTunnel(String::from("--proxy-tunnel")),
            CurlFlag::Trace(_) => CurlFlag::Trace(String::from("--trace")),
            CurlFlag::DumpHeaders(_) => CurlFlag::DumpHeaders(String::from("--dump-headers")),
            CurlFlag::DataUrlEncode(_) => CurlFlag::DataUrlEncode(String::from("--data-urlencode")),
            CurlFlag::Referrer(_) => CurlFlag::Referrer(String::from("-e")),
            CurlFlag::PreventDefaultConfig(_) => CurlFlag::PreventDefaultConfig(String::from("-q")),
            CurlFlag::FtpSsl(_) => CurlFlag::FtpSsl(String::from("--ftp-ssl")),
        }
    }
}
