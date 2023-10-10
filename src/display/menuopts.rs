/*
* Screen Menu Options
 */
use lazy_static::lazy_static;

pub const SAVED_COMMANDS_PARAGRAPH: &str = "\nPress q to exit\nPress Enter to Execute command\nPress 'ESC' to go back\nPress 'x' to Delete\n";
pub const CURL: &str = "curl";
pub const WGET: &str = "wget";
pub const CUSTOM: &str = "custom";
pub const DOWNLOAD: &str = "Download";
pub const API_KEY_PARAGRAPH: &str =
    "Press q to quit\nPress Enter to select\nPress 'x' to delete an item\n";
pub const HTTP_REQUEST: &str = "HTTP Request";
pub const DEFAULT_MENU_PARAGRAPH: &str =
    "\nPress q to exit \n Press Enter to select \n Please select a Menu item\n keybindings to navigate";
pub const API_KEY_TITLE: &str = "My API Keys";
pub const METHOD_MENU_TITLE: &str = "** CuTE ** Choose a Method";
pub const SAVED_COMMANDS_TITLE: &str = "My Saved cURL Commands";
pub const DEFAULT_MENU_TITLE: &str = "** CuTE **";
pub const AUTH_MENU_TITLE: &str = "** CuTE ** Authentication Menu 󰌋";
pub const VIEW_BODY_TITLE: &str = "** CuTE ** View Response Body";
pub const INPUT_MENU_TITLE: &str = "** CuTE ** Input **";
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
/*
* define_curl_flags! {
    Verbose("-v"),
    Output("-o"),
    User("-u"),
    Bearer("-H"),
    Headers("-H"),
    Digest("--digest"),
    Basic("-H"),
    AnyAuth("--any-auth"),
    UnixSocket("--unix-socket"),
    UploadFile("--upload-file"),
    Ntlm("--ntlm"),
    NtlmWb("--ntlm-wb"),
    Proxy("-x"),
    AwsSigv4("--aws-sigv4"),
    ProxyTunnel("--proxy-tunnel"),
    Socks5("--socks5"),
    File("-F"),
    FtpAccount("--ftp-account"),
    FtpSsl("--ftp-ssl"),
    Trace("--trace"),
    DataUrlEncode("--data-urlencode"),
    DumpHeaders("--dump-headers"),
    Referrer("-e"),
    Insecure("--insecure"),
    PreventDefaultConfig("-q"),
    CaCert("--cacert"),
    CaNative("--ca-native"),
    CaPath("--capath"),
    SpnegoAuth("--negotiate"),
    Kerberos("--krb"),
    Progress("--progress-bar"),
}
*/
#[rustfmt::skip]
// Yeah... if this is normal here, it f**ks up when we try to center it on the screen
pub const CUTE_LOGO: &str =
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
pub const CUTE_LOGO2: &str = "
 @@@@@@@. @@@  @@@  @@@@@@@  @@@@@@@@  
@@@@@@@@. @@@  @@@  @@@@@@@  @@@@@@@@  
!@@ ````  @@!  @@@    @@!    @@!       
   !@!'       !@!  @!@    !@!    !@!       
  !@!'      @!@  !@!    @!!    @!!!:!    
 !!!'      !@!  !!!    !!!    !!!!!:    
:!!'      !!:  !!!    !!:    !!:       
:!:'.. .  :!:  !:!    :!:    :!:       
 ::: :::' ::::: ::     ::     :: ::::  
 :: :: :'  : :  :      :     : :: ::   
                                       ";

pub const DISPLAY_OPT_VERBOSE: &str = "- Verbose";
pub const DISPLAY_OPT_COMMAND_SAVED: &str = "- Command will be saved";
pub const DISPLAY_OPT_HEADERS: &str = "- Response headers included";
pub const DISPLAY_OPT_PROGRESS_BAR: &str = "- Progress Bar will be displayed";
pub const DISPLAY_OPT_FAIL_ON_ERROR: &str = "- Fail on error";
pub const DISPLAY_OPT_TOKEN_SAVED: &str = "- Token will be saved";

lazy_static! {
    pub static ref MAIN_MENU_OPTIONS: [&'static str; 4] = [
        "Build and send an HTTP request 󰖟 ",
        "Download a remote file or directory 󰧩 ",
        "View my stored API keys 󱂛  ",
        "View or execute my saved commands  ",
    ];
    pub static ref REQUEST_MENU_OPTIONS: [&'static str; 11] = [
        "Add a URL 󰖟 ",
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
    ];
    pub static ref DOWNLOAD_MENU_OPTIONS: [&'static str; 4] = [
        "Specify recursive depth 󰆙 ",
        "Add a URL 󰖟  ",
        "Specify output filepath  ",
        "Begin Download  ",
    ];
    pub static ref METHOD_MENU_OPTIONS: [&'static str; 6] = [
        "GET",
        "POST",
        "PUT",
        "DELETE",
        "PATCH",
        "OTHER (custom command)"
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
    pub static ref MORE_FLAGS_MENU: [&'static str; 14] = [
        "Follow Redirects (-L)",
        "Specify Max redirects (--max-redirs)",
        "Add Cookie (-b)",
        "Add Proxy-Tunnel (--proxy-tunnel)",
        "Specify Socks5 Proxy (--socks5)",
        "Add FTP Account  (--ftp-account)",
        "Add FTP SSL (--ftp-ssl)",
        "Specify Referrer (-e)",
        "Data URL Encode (--data-urlencode)",
        "Add Trace (--trace)",
        "Add Ca-Cert (--cacert)",
        "Add Ca-Native (--ca-native)",
        "Specify Ca-Path (--capath)",
        "Add Progress Bar (--progress-bar)",
    ];
    pub static ref RESPONSE_MENU_OPTIONS: [&'static str; 4] = [
        "Write to file? 󱇧 ",
        "View response headers 󰰀 ",
        "View response body 󰈮 ",
        "Copy command to clipboard 󰅎 "
    ];
}
