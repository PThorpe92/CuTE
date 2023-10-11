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
pub enum AppOptions {
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

impl AppOptions {
    pub fn replace_value(&mut self, val: String) {
        match self {
            AppOptions::Headers(key) => {
                *key = val;
            }
            AppOptions::URL(url) => {
                *url = val;
            }
            AppOptions::Outfile(outfile) => {
                *outfile = val;
            }
            AppOptions::Response(response) => {
                *response = val;
            }
            AppOptions::RecDownload(level) => {
                *level = val.parse::<usize>().unwrap();
            }
            AppOptions::Auth(auth) => {
                *auth = val;
            }
            AppOptions::UnixSocket(socket) => {
                *socket = val;
            }
            AppOptions::Cookie(cookie) => {
                *cookie = val;
            }
            AppOptions::Referrer(referrer) => {
                *referrer = val;
            }
            AppOptions::CaPath(ca_cert) => {
                *ca_cert = val;
            }
            AppOptions::MaxRedirects(max_redirects) => {
                *max_redirects = val.parse::<usize>().unwrap();
            }
            AppOptions::UserAgent(ua) => {
                *ua = val;
            }

            _ => {}
        }
    }

    pub fn get_value(&self) -> String {
        match self {
            AppOptions::Verbose => String::from("Verbose"),
            AppOptions::Headers(key) => format!("{}", key),
            AppOptions::URL(url) => url.clone(),
            AppOptions::Outfile(outfile) => outfile.clone(),
            AppOptions::SaveCommand => String::from("Save Command"),
            AppOptions::Response(response) => response.clone(),
            AppOptions::RecDownload(level) => level.to_string(),
            AppOptions::Auth(auth) => auth.clone(),
            AppOptions::SaveToken => String::from("Save Token"),
            AppOptions::UnixSocket(socket) => socket.clone(),
            AppOptions::EnableHeaders => String::from("--include"),
            AppOptions::ProgressBar => String::from("--progress-bar"),
            AppOptions::FailOnError => String::from("--fail"),
            AppOptions::ProxyTunnel => String::from("--proxy-tunnel"),
            AppOptions::UserAgent(ua) => ua.clone(),
            AppOptions::MaxRedirects(max_redirects) => max_redirects.to_string(),
            AppOptions::Cookie(cookie) => cookie.clone(),
            AppOptions::Referrer(referrer) => referrer.clone(),
            AppOptions::CaPath(path) => path.clone(),
            AppOptions::CertInfo => String::from("--cert-info"),
            AppOptions::FollowRedirects => String::from("--location"),
            AppOptions::MatchWildcard => String::from("--glob"),
            AppOptions::TcpKeepAlive => String::from("--tcp-keepalive"),
            AppOptions::UnrestrictedAuth => String::from("--unrestricted-auth"),
        }
    }
}
