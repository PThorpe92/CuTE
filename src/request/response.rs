use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn from_raw_string(response_str: &str) -> Result<Response, &'static str> {
        // Split the response into lines
        let lines: Vec<&str> = response_str.lines().collect();

        // Ensure there's at least one line in the response
        if lines.is_empty() {
            return Err("Empty response");
        }

        // Parse the first line to extract the status code
        let first_line = lines[0].trim();
        let status_code = first_line
            .split_whitespace()
            .nth(1)
            .and_then(|status| status.parse::<u16>().ok())
            .ok_or("Invalid status code")?;

        // HashMap for the headers
        let mut headers = HashMap::new();
        let mut body = String::new();

        // Iterate through the remaining lines to extract headers and body
        let mut parsing_body = false;
        for line in &lines[1..] {
            if parsing_body {
                body.push_str(line);
                body.push('\n');
            } else if line.is_empty() {
                // An empty line separates headers from the body
                parsing_body = true;
            } else {
                // Parse headers in key-value format (will happen first)
                if let Some((key, value)) = line.split_once(':') {
                    headers.insert(key.trim().to_string(), value.trim().to_string());
                }
            }
        }

        Ok(Response {
            status: status_code,
            headers,
            body,
        })
    }

    pub fn get_headers(&self) -> serde_json::Value {
        serde_json::to_value(&self.headers).unwrap()
    }
}
