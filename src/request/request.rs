use std::fs::File;
use std::io::{BufWriter, Write};

use reqwest::{Client, Method};

use crate::request::auth::Auth;
use crate::request::methods::GET;

#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub method: String,
    // get, post, put, delete
    pub url: String,
    // the url to send the request to
    pub headers: Option<Vec<(String, String)>>,
    // header collection in (key, value) pairs
    pub body: Option<String>,
    // the body to send
    pub timeout: u32,
    // how long to wait for a response
    pub auth: Auth,
    // basic, bearer, digest, custom
    pub output: Option<String>, // where to write the output
}

impl Request {
    pub fn default() -> Self {
        Request {
            method: String::from(GET),
            url: String::new(),
            headers: None,
            body: None,
            timeout: 30,
            auth: Auth::AnyAuth,
            output: None,
        }
    }
    pub fn add_url(&mut self, url: String) {
        self.url = url.clone();
    }

    pub fn add_method(&mut self, method: String) {
        self.method = method.clone();
    }

    pub fn set_response(&mut self, response: String) {
        self.output = Some(response.clone());
    }

    pub fn add_headers(&mut self, headers: (String, String)) {
        match self.headers {
            Some(ref mut vec) => {
                let mut new_vec = vec.clone();
                new_vec.push(headers);
                self.headers = Some(new_vec);
            }
            None => {
                let mut vec = Vec::new();
                vec.push(headers);
                self.headers = Some(vec);
            }
        }
    }

    pub async fn send_request(&self) -> Result<String, reqwest::Error> {
        // Create a reqwest Client
        let client = Client::new();

        // Create the request builder based on the request type
        let mut request = match self.method.clone().as_str() {
            "GET" => client.request(Method::GET, self.url.clone()),
            "POST" => client.request(Method::POST, self.url.clone()),
            "PUT" => client.request(Method::PUT, self.url.clone()),
            "DELETE" => client.request(Method::DELETE, self.url.clone()),
            "PATCH" => client.request(Method::PATCH, self.url.clone()),
            _ => client.request(Method::GET, self.url.clone()),
        };

        // Set headers
        if let Some(headers) = &self.headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }

        // Set authentication
        match &self.auth {
            Auth::Basic(val) => {
                // Implement basic authentication
                request = request.basic_auth(val, Some(""));
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
        let status = response.status().as_u16();
        let res = response.text().await?;
        match status {
            200 => {
                if self.output.is_some() {
                    let file =
                        File::create(self.output.clone().unwrap()).expect("file creation failed");
                    let mut writer = BufWriter::new(file);
                    let _ = writer.write_all(res.clone().as_bytes()).unwrap_or_default();
                }
                return Ok(res.clone());
            }
            401 => {
                // do digest auth stuff here
            }
            _ => {}
        }
        return Ok(res.clone());
    }
}
