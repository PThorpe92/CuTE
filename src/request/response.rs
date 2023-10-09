use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Response {
    pub status: u16,
    pub headers: Vec<String>,
    pub body: String,
}

impl Response {
    pub fn get_headers(&self) -> Vec<String> {
        self.headers.clone()
    }
    pub fn from_raw_string(json_str: String) -> Self {
        let resp: Response = serde_json::from_str(&json_str).unwrap_or_else(|_| Response {
            status: 0,
            headers: Vec::new(),
            body: String::new(),
        });
        return resp;
    }
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
