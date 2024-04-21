use crate::{
    request::curl::{AuthKind, Method},
    screens::Screen,
};
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
    CmdLabel(i32),
    CmdDescription(i32),
    CollectionDescription(i32),
    ImportCollection,
    RenameCollection(i32),
    RequestError(String),
    AlertMessage(String),
    Method(Method),
}

impl InputOpt {
    pub fn get_return_screen(&self) -> Screen {
        match self {
            InputOpt::KeyLabel(_) => Screen::SavedKeys(None),
            InputOpt::CmdLabel(id) => Screen::SavedCommands {
                id: Some(*id),
                opt: None,
            },
            InputOpt::CaPath => Screen::RequestMenu(None),
            InputOpt::CaCert => Screen::RequestMenu(None),
            InputOpt::CookiePath => Screen::RequestMenu(None),
            InputOpt::CookieJar => Screen::RequestMenu(None),
            InputOpt::CookieExpires(_) => Screen::RequestMenu(None),
            InputOpt::CmdDescription(id) => Screen::SavedCommands {
                id: Some(*id),
                opt: None,
            },
            InputOpt::ApiKey => Screen::SavedKeys(None),
            InputOpt::UnixSocket => Screen::RequestMenu(None),
            InputOpt::UserAgent => Screen::RequestMenu(None),
            InputOpt::MaxRedirects => Screen::RequestMenu(None),
            InputOpt::NewCookie => Screen::RequestMenu(None),
            InputOpt::Referrer => Screen::RequestMenu(None),
            InputOpt::FtpAccount => Screen::RequestMenu(None),
            InputOpt::VerifyPeer => Screen::RequestMenu(None),
            InputOpt::Method(_) => Screen::RequestMenu(None),
            InputOpt::RequestError(_) => Screen::RequestMenu(None),
            InputOpt::AlertMessage(_) => Screen::RequestMenu(None),
            InputOpt::ImportCollection => Screen::SavedCollections(None),
            InputOpt::RenameCollection(_) => Screen::SavedCollections(None),
            InputOpt::Execute => Screen::Response(String::new()),
            InputOpt::CollectionDescription(_) => Screen::SavedCollections(None),
            InputOpt::URL => Screen::RequestMenu(None),
            InputOpt::UploadFile => Screen::RequestMenu(None),
            InputOpt::Headers => Screen::Headers,
            InputOpt::Output => Screen::Response(String::new()),
            InputOpt::Verbose => Screen::RequestMenu(None),
            InputOpt::RequestBody => Screen::RequestMenu(None),
            InputOpt::Auth(_) => Screen::RequestMenu(None),
            InputOpt::CookieValue(_) => Screen::RequestMenu(None),
        }
    }
    pub fn is_error(&self) -> bool {
        matches!(self, InputOpt::RequestError(_) | InputOpt::AlertMessage(_))
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
            InputOpt::CmdLabel(_) => write!(f, "| Command Label"),
            InputOpt::CmdDescription(_) => write!(f, "| Command Description"),
            InputOpt::CollectionDescription(_) => write!(f, "| Collection Description"),
        }
    }
}
