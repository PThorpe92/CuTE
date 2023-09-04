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

// Curl command builder
pub mod curl;

// Wget command builder
pub mod wget;

pub static GET: &str = "GET";
pub static POST: &str = "POST";
pub static PUT: &str = "PUT";
pub static DELETE: &str = "DELETE";
pub static PATCH: &str = "PATCH";
pub static HEAD: &str = "HEAD";
pub static OPTIONS: &str = "OPTIONS";

#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub cmd: CmdType,                                       // curl, wget, custom
    pub req_type: &'static str,                             // get, post, put, delete
    pub url: &'static str,                                  // the url to send the request to
    pub headers: Option<Vec<(&'static str, &'static str)>>, // header collection in (key, value) pairs
    pub body: Option<&'static str>,                         // the body to send
    pub timeout: u32,                                       // how long to wait for a response
    pub auth: Auth,                                         // basic, bearer, digest, custom
    pub output: Option<&'static str>,                       // where to write the output
}

impl Request {
    // default method is GET
    pub fn default(url: &str) -> Self {
        Request {
            cmd: CmdType::Curl,
            req_type: GET,
            url,
            headers: Some(vec![
                ("User-Agent", "Curl"),
                ("Content-Type", "application/json"),
            ]),
            body: None,
            timeout: 30,
            auth: Auth::AnyAuth,
            output: None,
        }
    }
    pub fn new(
        cmd: CmdType,
        req_type: &'static str,
        url: &'static str,
        headers: Option<Vec<(&'static str, &'static str)>>,
        body: Option<&'static str>,
        timeout: u32,
        auth: Auth,
        output: Option<&'static str>,
    ) -> Self {
        Request {
            cmd,
            req_type,
            url,
            headers,
            body,
            timeout,
            auth,
            output,
        }
    }
    pub fn add_url(&mut self, url: &'static str) {
        self.url = url;
    }
    pub fn add_headers(&mut self, headers: Vec<(&'static str, &'static str)>) {
        self.headers = Some(headers);
    }
    pub fn add_auth(&mut self, auth: Auth) {
        match auth {
            AnyAuth => self.auth = Auth::AnyAuth,
            // val in this case should be "username:password" string
            Auth::Basic(username_pwd) => {
                self.auth = Auth::Basic(username_pwd);
                self.add_headers(vec![("Authorization", "Basic")]);
                self.add_headers(vec![(username_pwd.as_str(), "")]);
            }
            Auth::Bearer(token) => {
                self.auth = Auth::Bearer(token);
                self.add_headers(vec![("Bearer: token", token.as_str())])
            }
            Auth::Digest(val) => {} // skip this for now...
            Auth::Custom(val) => self.auth = Auth::Custom(val),
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
            Auth::Basic(val) => {
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
pub enum Auth {
    AnyAuth,
    Basic(String),
    Bearer(String),
    Digest {
        username: String,
        password: String,
        realm: String,
        nonce: String,
        qop: String,
        nc: String,
        cnonce: String,
        uri: String,
        method: String,
    },
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

    pub fn from_headers(headers: Vec<(&str, &str)>) -> Self {
        let mut realm = String::new();
        let mut nonce = String::new();
        let mut qop = String::new();
        let mut nc = String::new();
        let mut cnonce = String::new();
        let mut uri = String::new();
        for (key, value) in headers {
            match key {
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
        pos: Option<&str>,
        digest: Option<DigestAuth>,
    ) -> Result<Auth, String> {
        match auth {
            "basic" => Ok(Auth::Basic(info.to_string())),
            "bearer" => Ok(Auth::Bearer(info.to_string())),
            "digest" => match digest {
                Some(digest) => Ok(Auth::Digest {
                    username: digest.username,
                    password: digest.password,
                    realm: digest.realm,
                    nonce: digest.nonce,
                    qop: digest.qop,
                    nc: digest.nc,
                    cnonce: digest.cnonce,
                    uri: digest.uri,
                    method: digest.method,
                }),
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
