/*
* String literals for Menus/Options
 */
pub const SAVED_COMMANDS_PARAGRAPH: &str =
    "\nPress q to exit\nPress Enter to Send Request\nPress 'ESC' or 'h' to go back\n";
pub const CURL: &str = "curl";
pub const WGET: &str = "wget";
pub const CUSTOM: &str = "custom";
pub const API_KEY_PARAGRAPH: &str =
    "Press q to quit\nPress 'ESC' or 'h' to go back\nPress Enter for Menu\n";
pub const HTTP_REQUEST: &str = "HTTP Request";
pub const DEFAULT_MENU_PARAGRAPH: &str =
    "\nPress q to exit. 'h' to go back \n Press Enter to select\n keybindings to navigate";
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
pub const ERROR_MENU_TITLE: &str = "* CuTE ** Error! *";
pub const SUCCESS_MENU_TITLE: &str = "* CuTE ** Success! *";
pub const POSTMAN_COLLECTION_TITLE: &str = "* CuTE ** Postman Collections";
pub const SUCCESS_MESSAGE: &str = "Request saved successfully";
pub const INPUT_OPT_URL: &str = "Enter a URL for your {}\n and press Enter";
pub const INPUT_OPT_HEADERS: &str =
    "MUST be \"Key:Value\" pair and press Enter \n Example: Content-Type: application/json";
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
!@!'      @!@  !@!    @!!    @!@!%!    
 !!'      !@!  !!!    !!!    !!!!!:    
:!!'      !!:  !!!    !!:    !!:       
:!:'.. .  :!:  !:!    :!:    :!:       
 ::: :::' ::::: ::     ::     :: ::::  
 :: :: :'  : :  :      :     : :: ::   
                                       ";

pub const DISPLAY_OPT_VERBOSE: &str = " Verbose";
pub const DISPLAY_OPT_COMMAND_SAVED: &str = " Request will be saved  ";
pub const DISPLAY_OPT_HEADERS: &str = " Response headers included 󱈝 ";
pub const DISPLAY_OPT_PROGRESS_BAR: &str = " Enable Progress Bar 󰦖 ";
pub const DISPLAY_OPT_FAIL_ON_ERROR: &str = " Fail on error  ";
pub const DISPLAY_OPT_TOKEN_SAVED: &str = " Token will be saved  ";
pub const DISPLAY_OPT_FOLLOW_REDIRECTS: &str = " Follow redirects 󱀀 ";
pub const DISPLAY_OPT_UNRESTRICTED_AUTH: &str = "- Send auth to hosts if redirected";
pub const DISPLAY_OPT_MAX_REDIRECTS: &str = " Max redirects: ";
pub const DISPLAY_OPT_UNIX_SOCKET: &str = "  Unix Socket: ";
pub const DISPLAY_OPT_CA_PATH: &str = "  󰄤 SSL Certificate path: ";
pub const DISPLAY_OPT_AUTH: &str = "  Auth: ";
pub const DISPLAY_OPT_MATCH_WILDCARD: &str = "  Match glob wildcard 󰛄 ";
pub const DISPLAY_OPT_CERT_INFO: &str = "  Request certificate info 󰄤 ";
pub const DISPLAY_OPT_BODY: &str = "  Request Body: ";
pub const DISPLAY_OPT_UPLOAD: &str = "  Upload file: ";
pub const DISPLAY_OPT_REQUEST_BODY: &str = "  Request Body";
pub const DISPLAY_OPT_TCP_KEEPALIVE: &str = "  Enable TCP keepalive 󰗶 ";
pub const DISPLAY_OPT_MAX_REC: &str = "  Specify recursive depth: ";
pub const DISPLAY_OPT_OUTFILE: &str = "  Specify output filepath: ";
pub const DISPLAY_OPT_REFERRER: &str = "  Specify Referrer: ";
pub const DISPLAY_OPT_COOKIE: &str = "  Cookie Path: ";
pub const DISPLAY_OPT_COOKIE_JAR: &str = "  Cookie Jar: ";
pub const DISPLAY_OPT_USERAGENT: &str = "  Specify User-Agent: ";
pub const DISPLAY_OPT_PROXY_TUNNEL: &str = "  Enable HTTP Proxy-Tunnel 󱠾 ";
pub const DISPLAY_OPT_URL: &str = "  Request URL: ";
pub const DISPLAY_OPT_CONTENT_HEADERS: &str = "  Headers: ";
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

pub const CMD_MENU_OPTIONS: [&str; 4] = [
    "Execute   ",
    "Delete   ",
    "Copy to Clipboard  󰅎 ",
    "Cancel   ",
];
pub const KEY_MENU_OPTIONS: [&str; 4] = [
    "Add a Label  ",
    "Delete   ",
    "Copy to Clipboard  󰅎 ",
    "Cancel   ",
];
pub const COLLECTION_MENU_OPTIONS: [&str; 4] = [
    "Import New Postman Collection 󰖟 ",
    "Create New Collection 󰖟 ",
    "View Collections 󱂛 ",
    "Cancel   ",
];
pub const ALERT_MENU_OPTIONS_KEY: [&str; 3] =
    ["Delete", "Copy Curl command to Clipboard", "Cancel"];
pub const MAIN_MENU_OPTIONS: [&str; 4] = [
    "Build and send an HTTP request 󰖟 ",
    "View saved requests  ",
    "View or Import Postman Collections",
    "View Saved API keys 󱂛  ",
];
pub const COLLECTION_ALERT_MENU_OPTS: [&str; 4] = [
    "View Requests in this collection",
    "Rename this collection",
    "Delete this collection",
    "Cancel",
];
pub const REQUEST_MENU_OPTIONS: [&str; 12] = [
    "Add a URL 󰖟 ",
    "Add a file for uploads  ",
    "Cookie options 󰆘 ",
    "Authentication 󰯄 ",
    "Header Options  ",
    "Enable verbose output [-v]",
    "Add Request Body 󰘦 ",
    "Save this Request  ",
    "Save your API token or login information  ",
    "Send Request  ",
    "More Options  ",
    "Clear all options  ",
];

pub const COOKIE_MENU_OPTIONS: [&str; 5] = [
    "Set Cookie file path (Use Cookies) 󰆘 ",
    "Set Cookie-Jar path (Storage) 󰆘 ",
    "Add New Cookie  󰆘 ",
    "Reset Cookie Session 󰆘 ",
    "Go back  ",
];

pub const METHOD_MENU_OPTIONS: [&str; 7] = [
    "OTHER (custom command)",
    "GET",
    "POST",
    "PUT",
    "DELETE",
    "PATCH",
    "HEAD",
];
pub const HEADER_MENU_OPTIONS: [&str; 5] = [
    "Add Custom Header 󰖟 ",
    "Add Content-Type: Application/Json  ",
    "Add Accept: Application/Json  ",
    "Enable Response Headers 󰰀 ",
    "Return to request menu  ",
];
pub const AUTHENTICATION_MENU_OPTIONS: [&str; 6] = [
    "Basic",
    "Bearer Token",
    "Digest",
    "AWS SignatureV4",
    "Ntlm",
    "SPNEGO",
];
pub const MORE_FLAGS_MENU: [&str; 12] = [
    "Follow Redirects 󱀀 ",
    "Specify Max redirects 󱀀 ",
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
pub const RESPONSE_MENU_OPTIONS: [&str; 5] = [
    "Write to file? 󱇧 ",
    "View response headers 󰰀 ",
    "View response body 󰈮 ",
    "Copy CLI command to clipboard 󰅎 ",
    "Return to main menu  ",
];
