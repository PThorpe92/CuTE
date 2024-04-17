use crate::request::curl::{AuthKind, Method};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum InputOpt {
    URL,
    UploadFile,
    Headers,
    Output,
    Verbose,
    RequestBody,
    Auth(AuthKind),
    VerifyPeer,
    Referrer,
    Execute,
    ApiKey,
    UnixSocket,
    UserAgent,
    MaxRedirects,
    CookiePath,
    NewCookie,
    CookieJar,
    CookieValue(String),   // store the name
    CookieExpires(String), // store the rest
    FtpAccount,
    CaPath,
    CaCert,
    KeyLabel(i32),
    ImportCollection,
    RenameCollection(i32),
    RequestError(String),
    AlertMessage(String),
    Method(Method),
}

impl InputOpt {
    pub fn is_error(&self) -> bool {
        matches!(self, InputOpt::RequestError(_))
    }
}

impl Display for InputOpt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputOpt::URL => write!(f, "| URL"),
            InputOpt::Headers => write!(f, "| Headers"),
            InputOpt::Output => write!(f, "| Output"),
            InputOpt::Referrer => write!(f, "| Referrer"),
            InputOpt::UploadFile => write!(f, "| Upload File"),
            InputOpt::Verbose => write!(f, "| Verbose"),
            InputOpt::RequestBody => write!(f, "| Request Body"),
            InputOpt::Auth(auth) => write!(f, "|- Authentication: {}", auth),
            InputOpt::Execute => write!(f, "| Execute"),
            InputOpt::ApiKey => write!(f, "| API Key"),
            InputOpt::UnixSocket => write!(f, "| Unix Socket"),
            InputOpt::UserAgent => write!(f, "| User Agent"),
            InputOpt::MaxRedirects => write!(f, "| Max Redirects"),
            InputOpt::NewCookie => write!(f, "| Cookie"),
            InputOpt::CookiePath => write!(f, "| Cookie"),
            InputOpt::CookieValue(_) => write!(f, "| Cookie Val"),
            InputOpt::CookieExpires(_) => write!(f, "| Cookie Expires"),
            InputOpt::CaPath => write!(f, "| Ca Path"),
            InputOpt::CaCert => write!(f, "| Ca Cert"),
            InputOpt::VerifyPeer => write!(f, "| Verify Peer DNS-Over-HTTPS"),
            InputOpt::FtpAccount => write!(f, "| FTP Account"),
            InputOpt::KeyLabel(_) => write!(f, "| Key Label"),
            InputOpt::ImportCollection => write!(f, "| Import Collection"),
            InputOpt::RenameCollection(_) => write!(f, "| Rename Collection"),
            InputOpt::RequestError(ref err) => write!(f, "| Error: {}", err),
            InputOpt::Method(method) => write!(f, "| Method: {}", method),
            InputOpt::CookieJar => write!(f, "| Cookie Jar"),
            InputOpt::AlertMessage(msg) => write!(f, "| Alert: {}", msg),
        }
    }
}
