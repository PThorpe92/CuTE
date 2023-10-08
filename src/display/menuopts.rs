/*
* Screen Menu Options
 */
use lazy_static::lazy_static;

pub const SAVED_COMMANDS_PARAGRAPH: &'static str = "\nPress q to exit\nPress Enter to Execute command\nPress 'ESC' to go back\nPress 'x' to Delete\n";
pub const CURL: &'static str = "curl";
pub const WGET: &'static str = "wget";
pub const CUSTOM: &'static str = "custom";
pub const DOWNLOAD: &'static str = "Download";
pub const API_KEY_PARAGRAPH: &'static str =
    "Press q to quit\nPress Enter to select\nPress 'x' to delete an item\n";
pub const HTTP_REQUEST: &'static str = "HTTP Request";
pub const DEFAULT_MENU_PARAGRAPH: &'static str =
    "\nPress q to exit \n Press Enter to select \n Please select a Menu item\n keybindings to navigate";
pub const API_KEY_TITLE: &'static str = "My API Keys";
pub const METHOD_MENU_TITLE: &'static str = "** CuTE ** Choose a Method";
pub const SAVED_COMMANDS_TITLE: &'static str = "My Saved cURL Commands";
pub const DEFAULT_MENU_TITLE: &'static str = "** CuTE **";
pub const AUTH_MENU_TITLE: &'static str = "** CuTE ** Authentication Menu 󰌋";
pub const VIEW_BODY_TITLE: &'static str = "** CuTE ** View Response Body";
pub const INPUT_MENU_TITLE: &'static str = "** CuTE ** Input **";
pub const DOWNLOAD_MENU_TITLE: &'static str = "* CuTE ** Downloads *";
pub const ERROR_MENU_TITLE: &'static str = "* CuTE ** Error! *";
pub const SUCCESS_MENU_TITLE: &'static str = "* CuTE ** Success! *";
pub const SUCCESS_MESSAGE: &'static str = "Command saved successfully";
pub const INPUT_OPT_URL: &'static str = "Enter a URL for your {}\n and press Enter";
pub const INPUT_OPT_HEADERS: &'static str =
    "MUST be \"Key:Value\" pair and press Enter \n Example: Content-Type: application/json";
pub const INPUT_OPT_REC_DOWNLOAD: &'static str =
    "Enter the recursion level and press Enter \n Example: 2";
pub const INPUT_OPT_AUTH_BASIC: &'static str = "Enter username:password and press Enter";
pub const INPUT_OPT_AUTH_ANY: &'static str = "Enter your username and press Enter";
pub const INPUT_OPT_AUTH_BEARER: &'static str = "Enter your API token and press Enter";
pub const INPUT_OPT_BASIC: &'static str = "Enter a value and press Enter";

// This padds the choices in the menu. This is the least hideous way to do this.(I think)
pub const OPTION_PADDING_MAX: &'static str = "\n\n\n\n";
pub const OPTION_PADDING_MID: &'static str = "\n\n\n";
pub const OPTION_PADDING_MIN: &'static str = "\n\n";

#[rustfmt::skip]
// Yeah... if this is normal here, it f**ks up when we try to center it on the screen
pub const CUTE_LOGO: &'static str =
"   . . . . . . . .  .  .  .  . . .    . .p  o  w .. e  r e  d.  . ..b.y ..
      ***************` *;;;;;;;  ;;;;;, $$####################$`****************``l
     %%%%%%%%%%%%%%%` %%%%%``|  #####``%%%%%%%%%%%%%%%%%%%%%% '%%%%%%%%%%%%%%%%``i
      %%%%#``;;;;;;;`  %%%%%``|  %%%%#``| **`;; %%%%&*+`` **;;| %%%%%%`   %%%%%%``b
         %%%%#``| *.      %%%%%``|  %%%%#``| ~   ` %%%%$*+`  ` i   %%%%%%`===#####``  _*_
        %%%%#``| `   ___ %%%%%``|  %%%%#``| _*_   %%%%$*+`   -*-  %%%%%%%%%%####``    *
      %%%%#``````%%%```%%%%%`/;; %%%%#```|      %%%%$*+`        %%%%%%`   _____`c 
 _*_  %%%%%%%%%%%%%%``|%%%%%=====%%%%#$`|       %%%%&*+``*      %%%%%%``` %%%%%#`u 
  *   %%%%%%%%%%%%%%`/; %%%%%%%%%%%%%%%%/      *%%%%%%**`       %%%%%%$####%%%%%``r
      ***************l...**********$  **`... .. .***.... . . ...****************'.l
";

lazy_static! {
    pub static ref MAIN_MENU_OPTIONS: [&'static str; 4] = [
        "Build and send an HTTP request 󰖟 ",
        "Download a remote file or directory 󰧩 ",
        "View my stored API keys 󱂛  ",
        "View or execute my saved commands  ",
    ];
    pub static ref REQUEST_MENU_OPTIONS: [&'static str; 8] = [
        "Add a URL 󰖟 ",
        "Add Authentication 󰯄 ",
        "Add Headers  ",
        "Enable verbose output [-v]",
        "Add Request Body 󰘦 ",
        "Save this Command  ",
        "Save your API token or login information  ",
        "Execute command  ",
    ];
    pub static ref DOWNLOAD_MENU_OPTIONS: [&'static str; 4] = [
        "Specify recursive depth 󰆙 ",
        "Add a URL 󰖟  ",
        "Specify output filepath  ",
        "Begin Download  ",
    ];
    pub static ref METHOD_MENU_OPTIONS: [&'static str; 5] =
        ["GET", "POST", "PUT", "DELETE", "PATCH",];
    pub static ref AUTHENTICATION_MENU_OPTIONS: [&'static str; 8] = [
        "Basic",
        "Bearer",
        "Digest",
        "AWS SignatureV4",
        "Ntlm",
        "NtlmWb",
        "Kerberos",
        "SPNEGO",
    ];
    pub static ref RESPONSE_MENU_OPTIONS: [&'static str; 4] = [
        "Write to file? 󱇧 ",
        "View response headers 󰰀 ",
        "View response body 󰈮 ",
        "Copy command to clipboard 󰅎 "
    ];
}
