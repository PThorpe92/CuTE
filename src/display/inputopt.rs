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
    Execute,
    ApiKey,
}

impl Display for InputOpt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputOpt::URL(url) => write!(f, "|- URL - {}", url),
            InputOpt::Headers => write!(f, "| Headers"),
            InputOpt::Output => write!(f, "| Output"),
            InputOpt::Verbose => write!(f, "| Verbose"),
            InputOpt::RequestBody => write!(f, "| Request Body"),
            InputOpt::RecursiveDownload => write!(f, "Recursive Download"),
            InputOpt::Auth(auth) => write!(f, "|- Authentication: {}", auth),
            InputOpt::Execute => write!(f, "| Execute"),
            InputOpt::ApiKey => write!(f, "| API Key"),
        }
    }
}
