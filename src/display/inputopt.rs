use crate::screens::auth::AuthType;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum InputOpt {
    URL,
    UploadFile,
    Headers,
    Output,
    Verbose,
    RequestBody,
    Auth(AuthType),
    VerifyPeer,
    Referrer,
    Execute,
    ApiKey,
    UnixSocket,
    UserAgent,
    MaxRedirects,
    Cookie,
    FtpAccount,
    CaPath,
    CaCert,
    KeyLabel(i32),
    ImportCollection,
    CreateCollection,
    RenameCollection(i32),
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
            InputOpt::Cookie => write!(f, "| Cookie"),
            InputOpt::CaPath => write!(f, "| Ca Path"),
            InputOpt::CaCert => write!(f, "| Ca Cert"),
            InputOpt::VerifyPeer => write!(f, "| Verify Peer DNS-Over-HTTPS"),
            InputOpt::FtpAccount => write!(f, "| FTP Account"),
            InputOpt::KeyLabel(_) => write!(f, "| Key Label"),
            InputOpt::ImportCollection => write!(f, "| Import Collection"),
            InputOpt::CreateCollection => write!(f, "| Create Collection"),
            InputOpt::RenameCollection(_) => write!(f, "| Rename Collection"),
        }
    }
}
