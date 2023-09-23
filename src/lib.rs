use reqwest::{Client, Method, Response};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use tokio::runtime::Runtime;

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

// Curl command builder
pub mod curl;

// Wget command builder
pub mod wget;

// Response parser
pub mod response;

// Database
pub mod db;

pub static GET: &str = "GET";
pub static POST: &str = "POST";
pub static PUT: &str = "PUT";
pub static DELETE: &str = "DELETE";
pub static PATCH: &str = "PATCH";
pub static HEAD: &str = "HEAD";
pub static OPTIONS: &str = "OPTIONS";

#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub method: String,                         // get, post, put, delete
    pub url: String,                            // the url to send the request to
    pub headers: Option<Vec<(String, String)>>, // header collection in (key, value) pairs
    pub body: Option<String>,                   // the body to send
    pub timeout: u32,                           // how long to wait for a response
    pub auth: Auth,                             // basic, bearer, digest, custom
    pub output: Option<String>,                 // where to write the output
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
#[derive(Debug, Clone, PartialEq)]
pub enum Auth {
    AnyAuth,
    Basic(String),
    Bearer(String),
    Digest(DigestAuth),
    Custom(String),
    Ntlm(String),
    Spnego(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct DigestAuth {
    realm: String,
    nonce: String,
    qop: String,
    nc: String,
    cnonce: String,
    uri: String,
    username: String,
    password: String,
    method: String,
}
impl DigestAuth {
    pub fn new(realm: &str, nonce: &str, qop: &str, nc: &str, cnonce: &str, uri: &str) -> Self {
        DigestAuth {
            realm: realm.to_string(),
            nonce: nonce.to_string(),
            qop: qop.to_string(),
            nc: nc.to_string(),
            cnonce: cnonce.to_string(),
            uri: uri.to_string(),
            username: String::new(),
            password: String::new(),
            method: String::new(),
        }
    }

    // When we initiate a request that requires digest authentication from an HTTP server,
    // the response header will look like this:
    //
    // HTTP/1.1 401 Unauthorized
    // WWW-Authenticate: Digest realm="Example Realm", qop="auth", nonce="UniqueNonce", opaque="OpaqueValue"
    //
    // looks lke we need to calculate our next request's auth (represented by this DigestAuth struct) by parsing
    // the response headers using Sha256
    // so it seems an auth.rs file is in order... or a response.rs file and we can handle all the
    // response parsing there.

    pub fn from_headers(headers: HashMap<String, String>) -> Self {
        let mut realm = String::new();
        let mut nonce = String::new();
        let mut qop = String::new();
        let mut nc = String::new();
        let mut cnonce = String::new();
        let mut uri = String::new();
        for (key, value) in headers.iter() {
            match key.as_str() {
                "realm" => realm = value.to_string(),
                "nonce" => nonce = value.to_string(),
                "qop" => qop = value.to_string(),
                "nc" => nc = value.to_string(),
                "cnonce" => cnonce = value.to_string(),
                "uri" => uri = value.to_string(),
                _ => {}
            }
        }
        DigestAuth {
            realm,
            nonce,
            qop,
            nc,
            cnonce,
            uri,
            username: String::new(),
            password: String::new(),
            method: String::new(),
        }
    }
}

impl Auth {
    pub fn new(
        auth: &str,
        info: &str,
        //pos: Option<&str>,
        digest: Option<DigestAuth>,
    ) -> Result<Auth, String> {
        match auth {
            "basic" => Ok(Auth::Basic(info.to_string())),
            "bearer" => Ok(Auth::Bearer(info.to_string())),
            "digest" => match digest {
                Some(digest) => Ok(Auth::Digest(DigestAuth {
                    username: digest.username,
                    password: digest.password,
                    realm: digest.realm,
                    nonce: digest.nonce,
                    qop: digest.qop,
                    nc: digest.nc,
                    cnonce: digest.cnonce,
                    uri: digest.uri,
                    method: digest.method,
                })),
                None => Err("Digest authentication requires a username and password".to_string()),
            },
            "custom" => Ok(Auth::Custom(info.to_string())),
            "spnego" => Ok(Auth::Spnego(info.to_string())),
            "ntlm" => Ok(Auth::Ntlm(info.to_string())),

            _ => Ok(Auth::Basic(info.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CmdType {
    Curl,
    Wget,
    Custom,
}
