// This is Auth pertaining to actual requests

use std::collections::HashMap;

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
