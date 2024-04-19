use crate::display::{AppOptions, HeaderKind};

use self::curl::Curl;

pub mod curl;
// Response parser
pub mod response;

pub trait ExecuteOption {
    fn add_option(&mut self, opt: &AppOptions);
    fn remove_option(&mut self, opt: &AppOptions);
}

impl ExecuteOption for Curl {
    fn add_option(&mut self, opt: &AppOptions) {
        match opt {
            AppOptions::URL(ref url) => self.set_url(url),
            AppOptions::Outfile(ref file) => self.set_outfile(file),
            AppOptions::UploadFile(ref file) => self.set_upload_file(file),
            AppOptions::UnixSocket(ref file) => self.set_unix_socket(file),
            AppOptions::ProgressBar => self.enable_progress_bar(true),
            AppOptions::FailOnError => self.set_fail_on_error(true),
            AppOptions::Verbose => self.set_verbose(true),
            AppOptions::Response(ref resp) => self.set_response(resp),
            AppOptions::SaveCommand => self.save_command(true),
            AppOptions::SaveToken => self.save_token(true),
            AppOptions::FollowRedirects => self.set_follow_redirects(true),
            AppOptions::UnrestrictedAuth => self.set_unrestricted_auth(true),
            AppOptions::TcpKeepAlive => self.set_tcp_keepalive(true),
            AppOptions::ProxyTunnel => self.set_proxy_tunnel(true),
            AppOptions::CertInfo => self.set_cert_info(true),
            AppOptions::MatchWildcard => self.match_wildcard(true),
            AppOptions::CaPath(ref path) => self.set_ca_path(path),
            AppOptions::MaxRedirects(size) => self.set_max_redirects(*size),
            AppOptions::UserAgent(ref agent) => self.set_user_agent(agent),
            AppOptions::Referrer(ref s) => self.set_referrer(s),
            AppOptions::RequestBody(ref body) => self.set_request_body(body),
            AppOptions::CookieJar(ref jar) => self.set_cookie_jar(jar),
            AppOptions::CookiePath(ref path) => self.set_cookie_path(path),
            AppOptions::NewCookie(ref new) => self.add_cookie(new),
            AppOptions::NewCookieSession => self.reset_cookie_session(),
            AppOptions::Headers(ref headers) => self.add_headers(headers),
            AppOptions::Auth(auth) => self.set_auth(auth.clone()),
            AppOptions::EnableHeaders => self.enable_response_headers(true),
            AppOptions::ContentHeaders(ref headers) => self.set_content_header(headers),
        }
    }
    fn remove_option(&mut self, opt: &AppOptions) {
        match opt {
            AppOptions::URL(_) => self.set_url(""),
            AppOptions::Outfile(_) => self.set_outfile(""),
            AppOptions::UploadFile(_) => self.set_upload_file(""),
            AppOptions::UnixSocket(_) => self.set_unix_socket(""),
            AppOptions::ProgressBar => self.enable_progress_bar(false),
            AppOptions::FailOnError => self.set_fail_on_error(false),
            AppOptions::Verbose => self.set_verbose(false),
            AppOptions::Response(_) => self.set_response(""),
            AppOptions::SaveCommand => self.save_command(false),
            AppOptions::SaveToken => self.save_token(false),
            AppOptions::FollowRedirects => self.set_follow_redirects(false),
            AppOptions::UnrestrictedAuth => self.set_unrestricted_auth(false),
            AppOptions::TcpKeepAlive => self.set_tcp_keepalive(false),
            AppOptions::ProxyTunnel => self.set_proxy_tunnel(false),
            AppOptions::CertInfo => self.set_cert_info(false),
            AppOptions::MatchWildcard => self.match_wildcard(false),
            AppOptions::CaPath(_) => self.set_ca_path(""),
            AppOptions::MaxRedirects(_) => self.set_max_redirects(0),
            AppOptions::UserAgent(_) => self.set_user_agent(""),
            AppOptions::Referrer(_) => self.set_referrer(""),
            AppOptions::RequestBody(_) => self.set_request_body(""),
            AppOptions::CookieJar(_) => self.set_cookie_jar(""),
            AppOptions::CookiePath(_) => self.set_cookie_path(""),
            AppOptions::NewCookie(_) => self.add_cookie(""),
            AppOptions::NewCookieSession => self.reset_cookie_session(),
            AppOptions::Headers(_) => self.remove_headers(""),
            AppOptions::Auth(_) => self.set_auth(crate::request::curl::AuthKind::None),
            AppOptions::EnableHeaders => self.enable_response_headers(false),
            AppOptions::ContentHeaders(_) => self.set_content_header(&HeaderKind::None),
        }
    }
}
