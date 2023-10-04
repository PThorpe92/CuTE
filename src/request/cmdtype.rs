use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum CmdType {
    Curl,
    Wget,
}

impl Display for CmdType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            CmdType::Curl => write!(f, "HTTP Request"),
            CmdType::Wget => write!(f, "Download"),
        }
    }
}
