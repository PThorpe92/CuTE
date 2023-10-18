/*
* Screen Menu Options
 */
use lazy_static::lazy_static;

pub const SAVED_COMMANDS_PARAGRAPH: &str =
    "\nPress q to exit\nPress Enter to Execute command\nPress 'ESC' or 'h' to go back\n";
pub const CURL: &str = "curl";
pub const WGET: &str = "wget";
pub const CUSTOM: &str = "custom";
pub const DOWNLOAD: &str = "Download";
pub const API_KEY_PARAGRAPH: &str =
    "Press q to quit\nPress 'ESC' or 'h' to go back\nPress Enter for Menu\n";
pub const HTTP_REQUEST: &str = "HTTP Request";
pub const DEFAULT_MENU_PARAGRAPH: &str =
    "\nPress q to exit \n Press Enter to select \n Please select a Menu item\n keybindings to navigate";
pub const AWS_AUTH_MSG: &str =
    "Alert: AWS Signature V4 Authentication is using the following ENV VARs:
    \nAWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, AWS_REGION";
pub const AWS_AUTH_ERROR_MSG: &str =
    "Error: AWS Signature V4 Authentication requires the following ENV VARs:
    \nAWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, AWS_REGION";
pub const API_KEY_TITLE: &str = "My API Keys";
pub const METHOD_MENU_TITLE: &str = "** CuTE ** Choose a Method";
pub const SAVED_COMMANDS_TITLE: &str = "My Saved cURL Commands";
pub const DEFAULT_MENU_TITLE: &str = "** CuTE **";
pub const AUTH_MENU_TITLE: &str = "** CuTE ** Authentication Menu 󰌋";
pub const VIEW_BODY_TITLE: &str = "** CuTE ** View Response Body";
pub const INPUT_MENU_TITLE: &str = "** Press i to enter Insert mode **";
pub const DOWNLOAD_MENU_TITLE: &str = "* CuTE ** Downloads *";
pub const ERROR_MENU_TITLE: &str = "* CuTE ** Error! *";
pub const SUCCESS_MENU_TITLE: &str = "* CuTE ** Success! *";
pub const SUCCESS_MESSAGE: &str = "Command saved successfully";
pub const INPUT_OPT_URL: &str = "Enter a URL for your {}\n and press Enter";
pub const INPUT_OPT_HEADERS: &str =
    "MUST be \"Key:Value\" pair and press Enter \n Example: Content-Type: application/json";
pub const INPUT_OPT_REC_DOWNLOAD: &str = "Enter the recursion level and press Enter \n Example: 2";
pub const INPUT_OPT_AUTH_BASIC: &str = "Enter username:password and press Enter";
pub const INPUT_OPT_AUTH_ANY: &str = "Enter your username and press Enter";
pub const INPUT_OPT_AUTH_BEARER: &str = "Enter your API token and press Enter";
pub const INPUT_OPT_BASIC: &str = "Enter a value and press Enter";
// This padds the choices in the menu. This is the least hideous way to do this.(I think)
pub const OPTION_PADDING_MAX: &str = "\n\n\n\n";
pub const OPTION_PADDING_MID: &str = "\n\n\n";
pub const OPTION_PADDING_MIN: &str = "\n\n";
pub const NEWLINE: &str = "\n";
// Yeah... if this is normal here, it f**ks up when we try to center it on the screen
#[rustfmt::skip]
pub const CUTE_LOGO: &str =
"    . . . . . . .  .  .  .  . . .    . .p  o  w .. e  r e  d.  . ..b.y ..
     ``*`*`*`*``*``*` .uuuu.    uuuuu, ``*`*'*'*'*''*'`*`*`*`*`'*'*'`**`'*'*'*`*`l             *&%&*   *&%&*
    $#$#$#$#$#$#$#$` $#$#$``|  #$#$#$''$#$#$#$#$#$#$#$#$#$#$ '#%#%#%#%#%#%#%##``i   **       *&%&*   *&%&*
     %%%%#``;;;;;;;`  %%%%#``|  %%%%#``| **`;; %%%%&*+`` **;;| %%%%%%`   %%%%%%``b  *%%*     *&%&*   *&%&*
     %%%%#``| *.      %%%%#``|  %%%%#``| ~   ` %%%%$*+`  ` .   %%%%%%`===#####``     **     *&%&*   *&%&*
     %%%%#``| `   ___ %%%%#``|  %%%%#``| _*_   %%%%$*+`   -*-  %%%%%%%%%%####``            *&%&*   *&%&*
     %%%%#``````%%%```%%%%#`/;; %%%%#```|      %%%%$*+`    |   %%%%%%`   _____`c     **   *&%&*   *&%&*
 _*_ %%%%%%%%%%%%%%``|%%%%#=====%%%%#$`|       %%%%&*+``*      %%%%%%``` %%%%%#`u   *%%* *&%&*   *&%&*
  *  %%%%%%%%%%%%%%`/; %%%%%%%%%%%%%%%%/      *%%%%%%**`       %%%%%%$####%%%%%``r   ** *&%&*   *&%&*
     ***************l...**********$  **`. .  . .***.. .   . . .****************'.l     *&%&*   *&%&*
";
#[rustfmt::skip]
pub const CUTE_LOGO2: &str = "
 @@@@@@@. @@@  @@@  @@@@@@@  @@@@@@@@  
@@@@@@@@. @@@  @@@  @@@@@@@  @@@@@@@@  
!@@ ````  @@!  @@@    @@!    @@!       
!@!'      !@!  @!@    !@!    !@!       
!@!'      @!@  !@!    @!!    @!!!:!    
 !!'      !@!  !!!    !!!    !!!!!:    
:!!'      !!:  !!!    !!:    !!:       
:!:'.. .  :!:  !:!    :!:    :!:       
 ::: :::' ::::: ::     ::     :: ::::  
 :: :: :'  : :  :      :     : :: ::   
                                       ";

pub const DISPLAY_OPT_VERBOSE: &str = " Verbose";
pub const DISPLAY_OPT_COMMAND_SAVED: &str = " Command will be saved  ";
pub const DISPLAY_OPT_HEADERS: &str = " Response headers included 󱈝 ";
pub const DISPLAY_OPT_PROGRESS_BAR: &str = " Enable Progress Bar 󰦖 ";
pub const DISPLAY_OPT_FAIL_ON_ERROR: &str = " Fail on error  ";
pub const DISPLAY_OPT_TOKEN_SAVED: &str = " Token will be saved  ";
pub const DISPLAY_OPT_FOLLOW_REDIRECTS: &str = " Follow redirects 󱀀 ";
pub const DISPLAY_OPT_UNRESTRICTED_AUTH: &str = "- Send auth to hosts if redirected";
pub const DISPLAY_OPT_MAX_REDIRECTS: &str = " Max redirects: ";
pub const DISPLAY_OPT_UNIX_SOCKET: &str = "  Unix Socket: ";
pub const DISPLAY_OPT_CA_PATH: &str = "  󰄤 SSL Certificate path: ";
pub const DISPLAY_OPT_AUTH: &str = "  Authentication: ";
pub const DISPLAY_OPT_MATCH_WILDCARD: &str = "  Match glob wildcard 󰛄 ";
pub const DISPLAY_OPT_CERT_INFO: &str = "  Request certificate info 󰄤 ";
pub const DISPLAY_OPT_UPLOAD: &str = "  Upload file: ";
pub const DISPLAY_OPT_TCP_KEEPALIVE: &str = "  Enable TCP keepalive 󰗶 ";
pub const DISPLAY_OPT_MAX_REC: &str = "  Specify recursive depth: ";
pub const DISPLAY_OPT_OUTFILE: &str = "  Specify output filepath: ";
pub const DISPLAY_OPT_REFERRER: &str = "  Specify Referrer: ";
pub const DISPLAY_OPT_COOKIE: &str = "  Add Cookie: ";
pub const DISPLAY_OPT_USERAGENT: &str = "  Specify User-Agent: ";
pub const DISPLAY_OPT_PROXY_TUNNEL: &str = "  Enable HTTP Proxy-Tunnel 󱠾 ";
pub const DISPLAY_OPT_URL: &str = "  Request URL: ";
pub const UPLOAD_FILEPATH_ERROR: &str =
    "Error: Invalid file path. Please enter an absolute path or a valid relative path.";
pub const SOCKET_ERROR: &str =
    "Error: Invalid socket file path. Please use an absolute path or a valid relative path.";
pub const PARSE_INT_ERROR: &str = "Error: Please enter a valid integer.";
pub const CERT_ERROR: &str =
    "Error: Invalid certificate file path. Please use an absolute path or a valid relative path.";
pub const HEADER_ERROR: &str = "Error: Invalid header. Please use the format \"Key:Value\".";
pub const SAVE_AUTH_ERROR: &str =
    "Error: You must have selected Authentication in order to save your token";
pub const VALID_COMMAND_ERROR: &str =
    "Error: Invalid command.\n You must add either a URL or Unix Socket to execute a command";

lazy_static! {
    pub static ref CMD_MENU_OPTIONS: [&'static str; 4] = [
        "Execute   ",
        "Delete   ",
        "Copy to Clipboard  󰅎 ",
        "Cancel   ",
    ];
    pub static ref KEY_MENU_OPTIONS: [&'static str; 4] = [
        "Add a new key  ",
        "Delete   ",
        "Copy to Clipboard  󰅎 ",
        "Cancel   ",
    ];
    pub static ref ALERT_MENU_OPTIONS_KEY: [&'static str; 3] =
        ["Delete", "Copy to Clipboard", "Cancel"];
    pub static ref MAIN_MENU_OPTIONS: [&'static str; 4] = [
        "Build and send an HTTP request 󰖟 ",
        "Download a remote file or directory 󰧩 ",
        "View my stored API keys 󱂛  ",
        "View or execute my saved commands  ",
    ];
    pub static ref REQUEST_MENU_OPTIONS: [&'static str; 13] = [
        "Add a URL 󰖟 ",
        "Add a file for uploads  ",
        "Add Unix Socket address 󰟩 ",
        "Add Authentication 󰯄 ",
        "Add Headers  ",
        "Enable verbose output [-v]",
        "Enable response Headers 󰃁 ",
        "Add Request Body 󰘦 ",
        "Save this Command  ",
        "Save your API token or login information  ",
        "Execute command  ",
        "More Options  ",
        "Clear all options  ",
    ];
    pub static ref DOWNLOAD_MENU_OPTIONS: [&'static str; 4] = [
        "Specify recursive depth 󰆙 ",
        "Add a URL 󰖟  ",
        "Specify output filepath  ",
        "Begin Download  ",
    ];
    pub static ref METHOD_MENU_OPTIONS: [&'static str; 7] = [
        "OTHER (custom command)",
        "GET",
        "POST",
        "PUT",
        "DELETE",
        "PATCH",
        "HEAD",
    ];
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
    pub static ref MORE_FLAGS_MENU: [&'static str; 13] = [
        "Follow Redirects 󱀀 ",
        "Specify Max redirects 󱀀 ",
        "Add Cookie  󰆘 ",
        "Enable HTTP Proxy-Tunnel  󱠾 ",
        "Unrestricted Auth  ",
        "Specify Referrer  󰆽 ",
        "Specify SSL Certificate path 󰄤 ",
        "Request Certificate Info 󰄤 ",
        "Add Progress Bar 󰦖 ",
        "Fail on Error  ",
        "Match wildcard 󰛄 ",
        "Specify User-Agent  󰖟 ",
        "Enable TCP keepalive 󰗶 ",
    ];
    pub static ref RESPONSE_MENU_OPTIONS: [&'static str; 4] = [
        "Write to file? 󱇧 ",
        "View response headers 󰰀 ",
        "View response body 󰈮 ",
        "Copy command to clipboard 󰅎 "
    ];
}
