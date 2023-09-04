use reqwest::{Client, Method};
use std::fs::File;
use std::io::{BufWriter, Write};
/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Event handler.
pub mod handler;

pub mod curl;

pub mod wget;

pub static GET: &str = "get";
pub static POST: &str = "post";
pub static PUT: &str = "put";
pub static DELETE: &str = "delete";
pub static PATCH: &str = "patch";
pub static HEAD: &str = "head";
pub static OPTIONS: &str = "options";

#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub cmd: CmdType,                                       // curl, wget, custom
    pub req_type: &'static str,                             // get, post, put, delete
    pub url: &'static str,                                  // the url to hit
    pub headers: Option<Vec<(&'static str, &'static str)>>, // header collection in (key, value) pairs
    pub body: Option<&'static str>,                         // the body to send
    pub timeout: u32,                                       // how long to wait for a response
    pub auth: &'static str,                                 // basic, bearer, digest, custom
    pub output: Option<&'static str>,                       // where to write the output
}

impl Request {
    pub fn default() -> Self {
        Request {
            cmd: CmdType::Curl,
            req_type: "get",
            url: "https://localhost",
            headers: Some(vec![
                ("User-Agent", "Curl"),
                ("Content-Type", "application/json"),
            ]),
            body: None,
            timeout: 30,
            auth: "basic",
            output: None,
        }
    }
    pub async fn send_request(&self) -> Result<(), reqwest::Error> {
        // Create a reqwest Client
        let client = Client::new();

        // Create the request builder based on the request type
        let mut request = match self.req_type.clone() {
            "GET" => client.request(Method::GET, self.url),
            "POST" => client.request(Method::POST, self.url),
            "PUT" => client.request(Method::PUT, self.url),
            "DELETE" => client.request(Method::DELETE, self.url),
            "PATCH" => client.request(Method::PATCH, self.url),
            "HEAD" => client.request(Method::HEAD, self.url),
            "OPTIONS" => client.request(Method::OPTIONS, self.url),
            _ => client.request(Method::GET, self.url),
        };

        // Set headers
        if let Some(headers) = &self.headers {
            for (key, value) in headers {
                request = request.header(*key, *value);
            }
        }

        // Set authentication
        match self.auth {
            "basic" => {
                // Implement basic authentication
            }
            "bearer" => {
                // Implement bearer authentication
            }
            "digest" => {
                // Implement digest authentication
            }
            "custom" => {
                // Implement custom authentication
            }
            _ => {}
        }

        // Set request timeout
        request = request.timeout(std::time::Duration::from_secs(self.timeout.into()));

        // Set request body if provided
        if let Some(body) = &self.body {
            request = request.body(body.to_string());
        }

        // Send the request and handle the response
        let response = request.send().await?;
        match self.output {
            Some(output) => {
                let mut file = BufWriter::new(File::create(output).unwrap());
                file.write_all(response.text().await?.as_bytes()).unwrap();
            }
            None => {
                println!("{}", response.text().await?);
            }
        }

        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum AuthType {
    Basic,
    Bearer,
    Digest,
    Custom,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CmdType {
    Curl,
    Wget,
    Custom,
}
