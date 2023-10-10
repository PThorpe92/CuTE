/*
* Display - This is For Structures That Represent Display Items
* Or Are Related To Display Items In Some Way
 */

// Input Options
pub mod inputopt;

// Menu Options
pub mod menuopts;

// AuthType
pub mod auth;

/// Here are the options that require us to display a box letting
/// the user know that they have selected that option.
#[derive(Debug, Clone, PartialEq)]
pub enum DisplayOpts {
    Verbose,
    // TODO: support more headers
    Headers(String),
    URL(String),
    Outfile(String),
    SaveCommand,
    Response(String),
    RecDownload(usize),
    Auth(String),
    SaveToken,
    UnixSocket(String),
    FollowRedirects,
    Cookie(String),
    EnableHeaders,
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
}

impl DisplayOpts {
    pub fn replace_value(&mut self, val: String) {
        match self {
            DisplayOpts::Headers(key) => {
                *key = val;
            }
            DisplayOpts::URL(url) => {
                *url = val;
            }
            DisplayOpts::Outfile(outfile) => {
                *outfile = val;
            }
            DisplayOpts::Response(response) => {
                *response = val;
            }
            DisplayOpts::RecDownload(level) => {
                *level = val.parse::<usize>().unwrap();
            }
            DisplayOpts::Auth(auth) => {
                *auth = val;
            }
            DisplayOpts::UnixSocket(socket) => {
                *socket = val;
            }
            DisplayOpts::Cookie(cookie) => {
                *cookie = val;
            }
            DisplayOpts::Referrer(referrer) => {
                *referrer = val;
            }
            DisplayOpts::CaPath(ca_cert) => {
                *ca_cert = val;
            }
            DisplayOpts::MaxRedirects(max_redirects) => {
                *max_redirects = val.parse::<usize>().unwrap();
            }
            DisplayOpts::UserAgent(ua) => {
                *ua = val;
            }

            _ => {}
        }
    }

    pub fn get_value(&self) -> String {
        match self {
            DisplayOpts::Verbose => String::from("Verbose"),
            DisplayOpts::Headers(key) => format!("{}", key),
            DisplayOpts::URL(url) => url.clone(),
            DisplayOpts::Outfile(outfile) => outfile.clone(),
            DisplayOpts::SaveCommand => String::from("Save Command"),
            DisplayOpts::Response(response) => response.clone(),
            DisplayOpts::RecDownload(level) => level.to_string(),
            DisplayOpts::Auth(auth) => auth.clone(),
            DisplayOpts::SaveToken => String::from("Save Token"),
            DisplayOpts::UnixSocket(socket) => socket.clone(),
            DisplayOpts::EnableHeaders => String::from("--include"),
            DisplayOpts::ProgressBar => String::from("--progress-bar"),
            DisplayOpts::FailOnError => String::from("--fail"),
            DisplayOpts::ProxyTunnel => String::from("--proxy-tunnel"),
            DisplayOpts::UserAgent(ua) => ua.clone(),
            DisplayOpts::MaxRedirects(max_redirects) => max_redirects.to_string(),
            DisplayOpts::Cookie(cookie) => cookie.clone(),
            DisplayOpts::Referrer(referrer) => referrer.clone(),
            DisplayOpts::CaPath(path) => path.clone(),
            DisplayOpts::CertInfo => String::from("--cert-info"),
            DisplayOpts::FollowRedirects => String::from("--location"),
            DisplayOpts::MatchWildcard => String::from("--glob"),
            DisplayOpts::TcpKeepAlive => String::from("--tcp-keepalive"),
            DisplayOpts::UnrestrictedAuth => String::from("--unrestricted-auth"),
        }
    }
}
