#[derive(Debug, Clone, PartialEq)]
pub enum InputOpt {
    URL,
    Headers,
    Output,
    Verbose,
    RequestBody,
    RecursiveDownload,
    Authentication,
    Execute,
}

impl InputOpt {
    pub fn to_string(&self) -> String {
        match self {
            InputOpt::URL => "URL",
            InputOpt::Headers => "Headers",
            InputOpt::Output => "Output",
            InputOpt::RequestBody => "Request Body",
            InputOpt::RecursiveDownload => "Recursive Download",
            InputOpt::Authentication => "Authentication",
            InputOpt::Execute => "Execute",
            InputOpt::Verbose => "Verbose",
        }
            .to_string()
    }
}
