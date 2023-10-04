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
                return format!("Authentication: {}", auth);
            }
            InputOpt::Execute => "Execute",
            InputOpt::Verbose => "Verbose",
        }
        .to_string()
    }
}
