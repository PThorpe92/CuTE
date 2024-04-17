use serde::{Deserialize, Serialize};

use self::menuopts::{
    DISPLAY_OPT_AUTH, DISPLAY_OPT_BODY, DISPLAY_OPT_CA_PATH, DISPLAY_OPT_CERT_INFO,
    DISPLAY_OPT_COMMAND_SAVED, DISPLAY_OPT_CONTENT_HEADERS, DISPLAY_OPT_COOKIE,
    DISPLAY_OPT_COOKIE_JAR, DISPLAY_OPT_FAIL_ON_ERROR, DISPLAY_OPT_FOLLOW_REDIRECTS,
    DISPLAY_OPT_HEADERS, DISPLAY_OPT_MATCH_WILDCARD, DISPLAY_OPT_MAX_REDIRECTS,
    DISPLAY_OPT_OUTFILE, DISPLAY_OPT_PROGRESS_BAR, DISPLAY_OPT_PROXY_TUNNEL, DISPLAY_OPT_REFERRER,
    DISPLAY_OPT_TCP_KEEPALIVE, DISPLAY_OPT_TOKEN_SAVED, DISPLAY_OPT_UNIX_SOCKET,
    DISPLAY_OPT_UNRESTRICTED_AUTH, DISPLAY_OPT_UPLOAD, DISPLAY_OPT_URL, DISPLAY_OPT_USERAGENT,
    DISPLAY_OPT_VERBOSE,
};
use crate::request::curl::AuthKind;
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum HeaderKind {
    // TODO: This should hold value of keys
    Accept,
    ContentType,
    None,
}
impl Display for HeaderKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HeaderKind::Accept => write!(f, "Accept: Application/json"),
            HeaderKind::ContentType => write!(f, "Content-Type: Application/json"),
            HeaderKind::None => write!(f, ""),
        }
    }
}
/*
* Display - This is For Structures That Represent Display Items
* Or Are Related To Display Items In Some Way
 */
// Input Options
pub mod inputopt;

// Menu Options
pub mod menuopts;

/// Here are the options that require us to display a box letting
/// the user know that they have selected that option.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AppOptions {
    Verbose,
    Headers(String),
    URL(String),
    Outfile(String),
    SaveCommand,
    Response(String),
    Auth(AuthKind),
    SaveToken,
    UnixSocket(String),
    FollowRedirects,
    CookieJar(String),
    CookiePath(String),
    EnableHeaders,
    ContentHeaders(HeaderKind),
    ProgressBar,
    FailOnError,
    ProxyTunnel,
    CaPath(String),
    CertInfo,
    UserAgent(String),
    Referrer(String),
    MatchWildcard,
    TcpKeepAlive,
    UnrestrictedAuth,
    MaxRedirects(usize),
    UploadFile(String),
    NewCookie(String),
    RequestBody(String),
    NewCookieSession,
}

impl AppOptions {
    pub fn get_curl_flag_value(&self) -> String {
        match self {
            Self::Verbose => "-v".to_string(),
            Self::Headers(ref str) => format!("-H {str}"),
            Self::UploadFile(ref file) => format!("-T {file}"),
            Self::Outfile(ref file) => format!("-o {file}"),
            Self::NewCookie(ref cookie) => format!("--cookie {cookie}"),
            Self::CookieJar(ref jar) => format!("--cookie-jar {jar}"),
            Self::CookiePath(ref path) => format!("--cookie {path}"),
            Self::Referrer(ref referrer) => format!("-e {referrer}"),
            Self::CaPath(ref path) => format!("--cacert {path}"),
            Self::MaxRedirects(ref size) => format!("--max-redirs {size}"),
            Self::UserAgent(ref ua) => format!("-A {ua}"),
            Self::RequestBody(ref body) => format!("-d {body}"),
            Self::NewCookieSession => "--junk-session-cookies".to_string(),
            Self::ProxyTunnel => "--proxy-tunnel".to_string(),
            Self::CertInfo => "--certinfo".to_string(),
            Self::FollowRedirects => "-L".to_string(),
            Self::UnixSocket(ref socket) => format!("--unix-socket {socket}"),
            Self::MatchWildcard => "-g".to_string(),
            Self::ProgressBar => "--progress-bar".to_string(),
            Self::Auth(ref kind) => match kind {
                AuthKind::Basic(ref login) => {
                    format!("-u {login}")
                }
                AuthKind::Digest(ref login) => {
                    format!("--digest -u {login}")
                }
                AuthKind::Ntlm => "--ntlm".to_string(),
                AuthKind::Bearer(ref token) => format!("-H 'Authorization: Bearer {token}'"),
                AuthKind::AwsSigv4 => "--aws-sigv4".to_string(),
                AuthKind::Spnego => "--spnego".to_string(),
                AuthKind::None => "".to_string(),
            },
            Self::ContentHeaders(ref kind) => match kind {
                HeaderKind::Accept => "-H \"Accept: Application/json\"".to_string(),
                HeaderKind::ContentType => "-H \"Content-Type: Application/json\"".to_string(),
                HeaderKind::None => "".to_string(),
            },
            Self::UnrestrictedAuth => "--anyauth".to_string(),
            _ => "".to_string(),
        }
    }
    pub fn should_toggle(&self) -> bool {
        matches!(
            self,
            Self::Verbose
                | Self::EnableHeaders
                | Self::ProgressBar
                | Self::FailOnError
                | Self::ProxyTunnel
                | Self::CertInfo
                | Self::FollowRedirects
                | Self::MatchWildcard
                | Self::TcpKeepAlive
                | Self::UnrestrictedAuth
        )
    }
    pub fn should_append(&self) -> bool {
        matches!(self, Self::Headers(_) | Self::NewCookie(_))
    }
    pub fn replace_value(&mut self, val: String) {
        match self {
            AppOptions::ContentHeaders(ref mut kind) => match val.as_str() {
                "Accept" => *kind = HeaderKind::Accept,
                "Content-Type" => *kind = HeaderKind::ContentType,
                _ => *kind = HeaderKind::None,
            },
            AppOptions::Headers(ref mut key) => {
                *key = val;
            }
            AppOptions::URL(ref mut url) => {
                *url = val;
            }
            AppOptions::Outfile(ref mut outfile) => {
                *outfile = val;
            }
            AppOptions::Response(ref mut response) => {
                *response = val;
            }
            AppOptions::UnixSocket(ref mut socket) => {
                *socket = val;
            }
            AppOptions::CookiePath(ref mut cookie) => {
                *cookie = val;
            }
            AppOptions::CookieJar(ref mut cookie) => {
                *cookie = val;
            }
            AppOptions::Referrer(ref mut referrer) => {
                *referrer = val;
            }
            AppOptions::CaPath(ref mut ca_cert) => {
                *ca_cert = val;
            }
            AppOptions::MaxRedirects(ref mut max_redirects) => {
                *max_redirects = val.parse::<usize>().unwrap();
            }
            AppOptions::UserAgent(ref mut ua) => {
                *ua = val;
            }
            AppOptions::UploadFile(ref mut file) => {
                *file = val;
            }
            AppOptions::RequestBody(ref mut body) => {
                *body = val;
            }
            _ => {}
        }
    }

    pub fn get_value(&self) -> String {
        match self {
            AppOptions::Verbose => String::from(DISPLAY_OPT_VERBOSE),
            AppOptions::URL(url) => format!("{}{}", DISPLAY_OPT_URL, url.clone()),
            AppOptions::Headers(val) => format!("{}{}", DISPLAY_OPT_HEADERS, val),
            AppOptions::Outfile(outfile) => format!("{}{}", DISPLAY_OPT_OUTFILE, outfile.clone()),
            AppOptions::SaveCommand => String::from(DISPLAY_OPT_COMMAND_SAVED),
            AppOptions::Response(response) => String::from(response),
            AppOptions::Auth(auth) => format!("{}{}", DISPLAY_OPT_AUTH, auth.clone()),
            AppOptions::SaveToken => String::from(DISPLAY_OPT_TOKEN_SAVED),
            AppOptions::UnixSocket(socket) => {
                format!("{}{}", DISPLAY_OPT_UNIX_SOCKET, socket.clone())
            }
            AppOptions::NewCookie(cookie) => format!("{}{}", DISPLAY_OPT_COOKIE, cookie.clone()),
            AppOptions::EnableHeaders => DISPLAY_OPT_HEADERS.to_string(),
            AppOptions::ProgressBar => String::from(DISPLAY_OPT_PROGRESS_BAR),
            AppOptions::FailOnError => String::from(DISPLAY_OPT_FAIL_ON_ERROR),
            AppOptions::ProxyTunnel => DISPLAY_OPT_PROXY_TUNNEL.to_string(),
            AppOptions::UserAgent(ua) => format!("{}{}", DISPLAY_OPT_USERAGENT, ua),
            AppOptions::MaxRedirects(max_redirects) => {
                format!("{}{}", DISPLAY_OPT_MAX_REDIRECTS, max_redirects)
            }
            AppOptions::NewCookieSession => String::from("New Cookie Session"),
            AppOptions::CookiePath(cookie) => format!("{}{}", DISPLAY_OPT_COOKIE, cookie.clone()),
            AppOptions::CookieJar(cookie) => {
                format!("{}{}", DISPLAY_OPT_COOKIE_JAR, cookie.clone())
            }
            AppOptions::Referrer(referrer) => {
                format!("{}{}", DISPLAY_OPT_REFERRER, referrer.clone())
            }
            AppOptions::CaPath(path) => format!("{}{}", DISPLAY_OPT_CA_PATH, path.clone()),
            AppOptions::CertInfo => DISPLAY_OPT_CERT_INFO.to_string(),
            AppOptions::FollowRedirects => DISPLAY_OPT_FOLLOW_REDIRECTS.to_string(),
            AppOptions::MatchWildcard => DISPLAY_OPT_MATCH_WILDCARD.to_string(),
            AppOptions::TcpKeepAlive => DISPLAY_OPT_TCP_KEEPALIVE.to_string(),
            AppOptions::UnrestrictedAuth => format!("{}{}", DISPLAY_OPT_UNRESTRICTED_AUTH, "󰄨"),
            AppOptions::UploadFile(file) => format!("{}{}", DISPLAY_OPT_UPLOAD, file.clone()),
            AppOptions::RequestBody(body) => format!("{}{}", DISPLAY_OPT_BODY, body.clone()),
            AppOptions::ContentHeaders(kind) => format!("{}{}", DISPLAY_OPT_CONTENT_HEADERS, kind),
        }
    }
}
