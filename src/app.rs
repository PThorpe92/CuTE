use std::{error, mem};

use lazy_static::lazy_static;
use tui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use tui_input::Input;

use crate::curl::{Curl, CurlFlag};
use crate::wget::Wget;
use crate::Request;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    pub cursor: usize,
    pub current_screen: Screen,
    pub screen_stack: Vec<Screen>,
    pub command: Option<Command<'a>>,
    pub selected: Option<usize>,
    pub opts: Vec<DisplayOpts>,
    pub input: Input,
    pub messages: Vec<String>,
    pub input_mode: InputMode,
    pub items: Vec<ListItem<'a>>,
    pub state: Option<ListState>,
    pub response: Option<String>,
    pub shareable_command: Option<ShareableCommand>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Screen {
    Home,
    Method(String),
    RequestMenu(String),
    InputMenu(InputOpt),
    Response(String),
    Success,
    Keys,
    Saved,
    Error(String),
    ViewBody,
}

impl<'a> Screen {
    pub fn default() -> Self {
        Screen::Home
    }

    pub fn get_opts(&self) -> Vec<ListItem<'a>> {
        match &self {
            Screen::Home => {
                return MAIN_MENU_OPTIONS
                    .iter()
                    .map(|i| ListItem::new(*i))
                    .collect();
            }
            Screen::Method(_) => {
                return METHOD_MENU_OPTIONS
                    .iter()
                    .map(|i| ListItem::new(*i))
                    .collect();
            }
            Screen::Keys => {
                return API_KEY_MENU_OPTIONS
                    .iter()
                    .map(|i| ListItem::new(*i))
                    .collect();
            }
            Screen::RequestMenu(_) => {
                return REQUEST_MENU_OPTIONS
                    .iter()
                    .map(|i| ListItem::new(*i))
                    .collect();
            }
            Screen::Saved => {
                return SAVED_COMMAND_OPTIONS
                    .iter()
                    .map(|i| ListItem::new(*i))
                    .collect();
            }
            Screen::Response(_) => {
                return RESPONSE_MENU_OPTIONS
                    .iter()
                    .map(|i| ListItem::new(*i))
                    .collect();
            }
            Screen::InputMenu(_) => {
                return INPUT_MENU_OPTIONS
                    .iter()
                    .map(|x| ListItem::new(*x))
                    .collect();
            }
            Screen::Success => {
                vec![ListItem::new("Success!").style(Style::default().fg(Color::Green))]
            }
            Screen::Error(_) => {
                vec![ListItem::new("Error!").style(Style::default().fg(Color::Red))]
            }
            Screen::ViewBody => {
                vec![ListItem::new("View Body").style(Style::default().fg(Color::Green))]
            }
        }
    }

    pub fn get_list(&self) -> List {
        List::new(self.get_opts())
            .block(
                Block::default()
                    .title(self.to_string().clone())
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol("->")
    }

    pub fn to_string(&self) -> String {
        match self {
            Screen::Home => "Main Menu",
            Screen::Method(_) => "Choose an HTTP Method",
            Screen::Keys => "My Saved API Keys",
            Screen::RequestMenu(_) => "Command Request Options",
            Screen::Saved => "My Saved Commands",
            Screen::InputMenu(_) => "User input",
            Screen::Response(_) => "Response",
            Screen::Success => "Success",
            Screen::Error(_) => "Error",
            Screen::ViewBody => "View response body",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShareableCommand {
    command: String,
    url: String,
    headers: Vec<(String, String)>,
    outfile: String,
    verbose: bool,
}

impl ShareableCommand {
    pub fn new() -> Self {
        Self {
            command: "".to_string(),
            url: "".to_string(),
            headers: Vec::new(),
            outfile: "".to_string(),
            verbose: false,
        }
    }

    pub fn set_command(&mut self, command: String) {
        self.command = command;
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn set_headers(&mut self, headers: Vec<(String, String)>) {
        self.headers = headers;
    }

    pub fn push_header(&mut self, header: (String, String)) {
        self.headers.push(header);
    }

    pub fn set_outfile(&mut self, outfile: String) {
        self.outfile = outfile;
    }

    pub fn render_command_str(&self) -> Option<String> {
        if self.command.is_empty() {
            return None;
        }

        if self.url.is_empty() {
            return None;
        }

        // This assembles the simplest possible command string
        let mut command_str = self.command.clone();

        // Check For Verbose Flag
        if self.verbose {
            // Verbose Flag Including Whitespace
            command_str.push_str(" -v");
        }

        // Whitespace
        command_str.push_str(" ");
        // URL
        command_str.push_str(&self.url);

        // Next We Check For Headers
        if self.headers.len() > 0 {
            for (key, value) in &self.headers {
                // Whitespace
                command_str.push_str("");
                // Header Flag
                command_str.push_str("-H ");
                // Open Quote
                command_str.push_str("\"");
                // Header Key
                command_str.push_str(&key);
                // Delimiter
                command_str.push_str(":");
                // Header Value
                command_str.push_str(&value);
                // Close Quote
                command_str.push_str("\"");
            }
        }

        // Check For Outfile
        if !self.outfile.is_empty() {
            // Whitespace
            command_str.push_str(" ");
            // Outfile Flag
            command_str.push_str(" -o ");
            // Outfile Name
            command_str.push_str(&self.outfile);
        }

        // Return Command String
        Some(command_str)
    }
}

/// Here are the options that require us to display a box letting
/// the user know that they have selected that option.
#[derive(Debug, Clone, PartialEq)]
pub enum DisplayOpts {
    Verbose,
    // TODO: support more headers
    Headers((String, String)),
    URL(String),
    Outfile(String),
    SaveCommand,
    Response(String),
    ShareableCmd(ShareableCommand),
}

#[derive(Debug)]
pub enum Command<'a> {
    Curl(Curl<'a>),
    Wget(Wget),
    Custom(Request),
}

impl<'a> Command<'a> {
    pub fn default(curl: Curl<'a>) -> Self {
        Command::Curl(curl)
    }

    pub fn set_method(&mut self, method: String) {
        match method.as_str() {
            "GET" => match self {
                Command::Curl(curl) => curl.set_get_method(true),
                Command::Wget(wget) => wget.set_method(method),
                Command::Custom(req) => req.method = method,
            },
            "POST" => match self {
                Command::Curl(curl) => curl.set_post_method(true),
                Command::Wget(wget) => wget.set_method(method),
                Command::Custom(req) => req.method = method,
            },
            "PUT" => match self {
                Command::Curl(curl) => curl.set_put_method(true),
                Command::Wget(wget) => wget.set_method(method),
                Command::Custom(req) => req.method = method,
            },
            "PATCH" => match self {
                Command::Curl(curl) => curl.set_patch_method(true),
                Command::Wget(wget) => wget.set_method(method),
                Command::Custom(req) => req.method = method,
            },
            "DELETE" => match self {
                Command::Curl(curl) => curl.set_delete_method(true),
                Command::Wget(wget) => wget.set_method(method),
                Command::Custom(req) => req.method = method,
            },
            _ => {}
        }
    }

    pub fn set_url(&mut self, url: String) {
        match self {
            Command::Curl(curl) => {
                curl.set_url(url.clone());
            }
            Command::Wget(wget) => {
                wget.set_url(url);
            }
            Command::Custom(req) => {
                req.url = url;
            }
        }
    }

    pub fn set_outfile(&mut self, file: String) {
        match self {
            Command::Curl(curl) => {
                curl.add_flag(CurlFlag::Output(""), Some(file));
            }
            Command::Wget(wget) => {
                wget.set_output(file);
            }
            Command::Custom(req) => {
                req.output = Some(file.clone());
            }
        }
    }

    pub fn add_headers(&mut self, headers: (String, String)) {
        match self {
            Command::Custom(req) => {
                req.add_headers(headers);
            }
            _ => {}
        }
    }
    pub fn set_headers(&mut self, headers: Vec<String>) {
        match self {
            Command::Curl(curl) => {
                curl.add_headers(headers);
            }
            Command::Custom(_) => {
                //req.set_headers(headers);
            }
            _ => {}
        }
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        match self {
            Command::Curl(curl) => curl.set_verbose(verbose),
            Command::Wget(wget) => {
                wget.set_verbose(verbose);
            }
            Command::Custom(_) => {}
        }
    }

    pub fn set_rec_download(&mut self, level: usize) {
        match self {
            Command::Wget(wget) => {
                wget.set_recursive_download(level as u8);
            }
            _ => {}
        }
    }

    pub async fn execute(&mut self) -> Result<String, std::io::Error> {
        match self {
            Command::Curl(curl) => Ok(curl.execute().unwrap_or("".to_string())),
            Command::Wget(wget) => Ok(wget.execute().unwrap_or("".to_string())),
            Command::Custom(req) => Ok(req.send_request().await.unwrap_or_default()),
        }
    }
    pub fn write_output(&mut self) -> Result<(), std::io::Error> {
        match self {
            Command::Curl(curl) => {
                curl.write_output()?;
                Ok(())
            }
            Command::Wget(wget) => {
                wget.write_output()?;
                Ok(())
            }
            Command::Custom(_) => Ok(()),
        }
    }

    pub fn set_response(&mut self, response: String) {
        match self {
            Command::Curl(curl) => {
                curl.set_response(response.clone());
            }
            Command::Wget(wget) => {
                wget.set_response(response.clone());
            }
            Command::Custom(req) => {
                req.set_response(response.clone());
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Command::Curl(_) => "Curl",
            Command::Wget(_) => "Wget",
            Command::Custom(_) => "Custom",
        }
        .to_string()
    }
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self {
            running: true,
            cursor: 0,
            screen_stack: vec![Screen::Home],
            selected: None,
            command: None,
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            opts: Vec::new(),
            items: Screen::Home.get_opts(),
            input: Input::default(),
            state: None,
            current_screen: Screen::Home,
            response: None,
            shareable_command: None,
        }
    }
}

impl<'a> App<'_> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn goto_screen(&mut self, screen: Screen) {
        self.screen_stack.push(screen.clone());
        self.current_screen = screen.clone();

        // If We Are Going Into A Screen Where There Would Be A Sharable Command
        // Lets Init That
        match self.current_screen {
            Screen::RequestMenu(_) => {
                self.shareable_command = Some(ShareableCommand::new());
            }
            _ => {} // Lets Add Cases As We Add Features
        }

        self.cursor = 0;
        self.items = screen.get_opts();
        self.selected = None;
    }

    pub fn go_back_screen(&mut self) {
        match self.screen_stack.pop() {
            // we are not returning to an input menu, so we pop the last element that wasn't an input menu
            Some(Screen::InputMenu(_)) => {
                // we can unwrap, because if we have hit an input menu, it's guaranteed
                self.current_screen = self.screen_stack.last().unwrap().clone();
            }
            Some(_) => match self.screen_stack.last() {
                Some(screen) => {
                    self.cursor = 0;
                    self.selected = None;
                    self.current_screen = screen.clone();
                }
                _ => {}
            },
            None => {}
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn move_cursor_down(&mut self) {
        if self.items.is_empty() || self.cursor >= self.items.len() {
            return;
        }
        if let Some(res) = self.cursor.checked_add(1) {
            self.cursor = res;
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.items.is_empty() {
            return;
        }
        if let Some(res) = self.cursor.checked_sub(1) {
            self.cursor = res;
        }
    }

    pub fn select_item(&mut self) {
        let state = self.state.as_mut().unwrap();
        if let Some(selected) = state.selected() {
            // ^^^ returns usize index
            self.selected = Some(selected);
        }
    }

    // Display option is some state that requires us to display the users
    // current selection on the screen so they know what they have selected
    // Lorenzo - Changing this because I dont think its doing what I want it to do.
    pub fn has_display_option(&self, opt: DisplayOpts) -> bool {
        // this is what we were doing before - self.opts.iter().any(|x| &x == &&opt)
        // I think this is what we want to do instead
        // We want to check if the option is in the vector
        // If it is, we want to return true
        // If it is not, we want to return false
        // We can do this by iterating over the vector and checking if the option is in the vector
        for element in self.opts.iter() {
            // I only care if its the same KIND of option, not the same value
            // This is annoying, I tried to do this an easier way

            match *element {
                DisplayOpts::URL(_) => {
                    if mem::discriminant(&opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::Headers(_) => {
                    if mem::discriminant(&opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::Outfile(_) => {
                    if mem::discriminant(&opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::Response(_) => {
                    if mem::discriminant(&opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::SaveCommand => {
                    if mem::discriminant(&opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::ShareableCmd(_) => {
                    if mem::discriminant(&opt) == mem::discriminant(element) {
                        return true;
                    }
                }
                DisplayOpts::Verbose => {
                    if mem::discriminant(&opt) == mem::discriminant(element) {
                        return true;
                    }
                }
            }
        }
        // Otherwise, its not there.
        false
    }

    // Lorenzo - Im adding this function as a slightly more
    // robust version of has_display_option, to test if we should be replacing a value or adding a new one
    fn should_replace_or_add(&self, opt: DisplayOpts) -> bool {
        // Lets match the type of display option
        // We know that only 1 URL should ever be added,
        // So if we're adding a URL we should replace it if it already exists
        match opt {
            DisplayOpts::URL(_) => !self.has_display_option(opt.clone()), // URL Should Be Replaced If It Already Exists
            DisplayOpts::Headers(_) => true, // Headers Should Be "Pushed" or Added
            DisplayOpts::Outfile(_) => !self.has_display_option(opt.clone()), // Outfile Should Be Replaced
            DisplayOpts::Verbose => !self.has_display_option(opt.clone()), // Verbose Should Be Toggled
            DisplayOpts::SaveCommand => !self.has_display_option(opt.clone()), // Save Command Should Be Toggled
            DisplayOpts::Response(_) => !self.has_display_option(opt.clone()), // Response Should Be Replaced
            DisplayOpts::ShareableCmd(_) => !self.has_display_option(opt.clone()), // Shareable Command Should Be Replaced With The New Version
            _ => false, // Anything Else Should Be Replaced.
        }
    }

    pub fn set_response(&mut self, response: String) {
        self.response = Some(response.clone());
        match self.command {
            Some(ref mut cmd) => {
                cmd.set_response(response);
            }
            None => {}
        }
    }

    // user selects once, we add. twice we remove.
    // Lorenzo - When a display option is added, we should also add it to the sharable command
    pub fn add_display_option(
        &mut self,
        mut opt: DisplayOpts, /* We Make This Mutable Because We Need To Compare It To A Mutable Reference Later */
    ) {
        if self.should_replace_or_add(opt.clone()) {
            // TRUE = We Should Add An Option
            // Adding The New Test Here. should_replace_or_add
            // The user has not yet added this command option yet, and so therefore, we push it to the opts vector.
            // I need a copy of the option before we move it
            let match_opt = opt.clone(); // Lets refactor all this later.
            self.opts.push(opt);
            // We also need to add it to the sharable command
            // but we need to know what kind of option it is
            // the best way I can think of right now is to match it but I'm sure there's a better way
            match match_opt {
                // Im cloning this to shut the borrow checker up, there is probably a better way
                DisplayOpts::Verbose => {
                    // Just add the verbose flag to the command
                    self.shareable_command.as_mut().unwrap().set_verbose(true);
                }
                DisplayOpts::Headers((key, value)) => {
                    // Push Header To Shareable Command
                    self.shareable_command
                        .as_mut()
                        .unwrap()
                        .push_header((key, value));
                }
                DisplayOpts::URL(url) => {
                    // Set URL To Sharable Command
                    self.shareable_command.as_mut().unwrap().set_url(url);
                }
                DisplayOpts::Outfile(outfile) => {
                    // Set Outfile To Shareable Command
                    self.shareable_command
                        .as_mut()
                        .unwrap()
                        .set_outfile(outfile);
                }
                _ => {
                    // Nothing
                    // This display opt does not factor into the sharable command.
                }
            }
        } else {
            // FALSE = We Should Replace An Option
            // The user has already added this command option, and so therefore, we should replace the old value with the new value.
            //self.opts.retain(|x| x != &opt);
            // Sorry, this is my way to do this idk if its the right way, but this is what makes sense to me in my head
            for element in self.opts.iter_mut() {
                // Same thing down here, I only care if its the same KIND of option, not the same value
                // Again, this is annoying, I tried to do this an easier way
                // but mem::discriminant doesnt like element as a comparison so I need to be particular
                // Sorry lets refactor this
                // TODO: Refactor This.

                // We Want To Just Replace A URL
                if let DisplayOpts::URL(_) = element {
                    *element = opt.clone(); // Copy The New URL Into The Old One
                    return;
                }

                // TODO: Headers Will Be Handled Differently.

                // TODO: Outfile Will Be Handled Differently.

                // TODO: Verbose & Save Command Will Be Handled Differently.

                // TODO: Other Shit
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputOpt {
    URL,
    Headers,
    Output,
    Verbose,
    RequestBody,
    RecursiveDownload,
    Authentication,
    Execute,
}

impl InputOpt {
    pub fn to_string(&self) -> String {
        match self {
            InputOpt::URL => "URL",
            InputOpt::Headers => "Headers",
            InputOpt::Output => "Output",
            InputOpt::RequestBody => "Request Body",
            InputOpt::RecursiveDownload => "Recursive Download",
            InputOpt::Authentication => "Authentication",
            InputOpt::Execute => "Execute",
            InputOpt::Verbose => "Verbose",
        }
        .to_string()
    }
}

pub enum Auth {
    Basic,
    Bearer,
    Digest,
    Hawk,
    OAuth,
    AWSSignature,
    NTLM,
    Kerberos,
    SPNEGO,
    Custom,
}

lazy_static! {
    pub static ref REQUEST_MENU_OPTIONS: [&'static str; 9] = [
        "Add a URL\n \n",
        "Add Authentication\n \n",
        "Add Headers\n \n",
        "Enable verbose output\n \n",
        "Specify request output file\n \n",
        "Add Request Body\n \n",
        "Save this command\n \n",
        "Recursive download (wget only)\n \n",
        "Execute command\n \n",
    ];
    pub static ref METHOD_MENU_OPTIONS: [&'static str; 5] = [
        "GET\n \n",
        "POST\n \n",
        "PUT\n \n",
        "DELETE\n \n",
        "PATCH\n \n",
    ];
    pub static ref AUTHENTICATION_MENU_OPTIONS: [&'static str; 10] = [
        "Basic\n \n",
        "Bearer\n \n",
        "Digest\n \n",
        "Hawk\n \n",
        "OAuth\n \n",
        "AWS Signature\n \n",
        "NTLM\n \n",
        "Kerberos\n \n",
        "SPNEGO\n \n",
        "Custom\n \n",
    ];
    pub static ref INPUT_MENU_OPTIONS: [&'static str; 3] = [
        "Please enter a URL for your request",
        "Please specify your request headers",
        "Please enter your request body",
    ];
    pub static ref MAIN_MENU_OPTIONS: [&'static str; 5] = [
        "Build and run a new cURL command\n  \n",
        "Build and run a new wget command\n  \n",
        "Build/send new custom HTTP request\n  \n",
        "View my stored API keys\n  \n",
        "View or execute my saved commands\n  \n",
    ];
    pub static ref RESPONSE_MENU_OPTIONS: [&'static str; 3] = [
        "Write to file?\n \n",
        "View response headers\n \n",
        "View response body\n \n",
    ];
    pub static ref API_KEY_MENU_OPTIONS: [&'static str; 3] = [
        "Add a new key\n \n",
        "View my keys\n \n",
        "Delete a key\n \n",
    ];
    pub static ref SAVED_COMMAND_OPTIONS: [&'static str; 3] = [
        "Add a new saved command\n \n",
        "View my saved commands\n \n",
        "Delete a saved command\n \n",
    ];
}
