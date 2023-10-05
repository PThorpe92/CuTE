use crate::request::cmdtype::CmdType;
use crate::request::curl::AuthKind;

#[derive(Debug, Clone, PartialEq)]
pub enum InputOpt {
    URL(CmdType),
    Headers,
    Output,
    Verbose,
    RequestBody,
    RecursiveDownload,
    Auth(AuthKind),
    Execute,
    ApiKey,
}

impl InputOpt {
    pub fn to_string(&self) -> String {
        match self {
            InputOpt::URL(_) => "URL",
            InputOpt::Headers => "Header",
            InputOpt::Output => "Output",
            InputOpt::RequestBody => "Request Body",
            InputOpt::RecursiveDownload => "Recursive Download",
            InputOpt::Auth(val) => {
                let auth = val.clone();
                return format!("Authentication: {}", auth.to_string());
            }
            InputOpt::Execute => "Execute",
            InputOpt::Verbose => "Verbose",
            InputOpt::ApiKey => "API Key",
        }
        .to_string()
    }
}
