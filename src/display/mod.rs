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
    EnableHeaders,
    ProgressBar,
    FailOnError,
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
        }
    }
}
