use std::fmt::Display;

use crate::request::cmdtype::CmdType;
use crate::screens::auth::AuthType;

#[derive(Debug, Clone, PartialEq)]
pub enum InputOpt {
    URL(CmdType),
    Headers,
    Output,
    Verbose,
    RequestBody,
    RecursiveDownload,
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
}

impl Display for InputOpt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputOpt::URL(url) => write!(f, "|- URL - {}", url),
            InputOpt::Headers => write!(f, "| Headers"),
            InputOpt::Output => write!(f, "| Output"),
            InputOpt::Referrer => write!(f, "| Referrer"),
            InputOpt::Verbose => write!(f, "| Verbose"),
            InputOpt::RequestBody => write!(f, "| Request Body"),
            InputOpt::RecursiveDownload => write!(f, "Recursive Download"),
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
        }
    }
}
