/*
* Screen Menu Options
 */
use lazy_static::lazy_static;

pub const SAVED_COMMANDS_PARAGRAPH: &'static str = "View / Delete my saved cURL commands.\nPress q to exit\nPress Enter to select\nPress h to go back\n Please select a Menu item\n";
pub const CURL: &'static str = "curl";
pub const WGET: &'static str = "wget";
pub const CUSTOM: &'static str = "custom";
pub const DOWNLOAD: &'static str = "Download";
pub const API_KEY_PARAGRAPH: &'static str = "Create / Edit / Delete API Keys and tokens.\nPress q to exit\nPress Enter to select\n Please select a Menu item\n";
pub const HTTP_REQUEST: &'static str = "HTTP Request";
pub const DEFAULT_MENU_PARAGRAPH: &'static str =
    "\nPress q to exit \n Press Enter to select \n Please select a Menu item\n";
pub const API_KEY_TITLE: &'static str = "My API Keys";
pub const METHOD_MENU_TITLE: &'static str = "** CuTE ** Choose a Method";
pub const SAVED_COMMANDS_TITLE: &'static str = "My Saved cURL Commands";
pub const DEFAULT_MENU_TITLE: &'static str = "** CuTE **";
pub const AUTH_MENU_TITLE: &'static str = "** CuTE ** Authentication Menu";
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
lazy_static! {
    pub static ref MAIN_MENU_OPTIONS: [&'static str; 4] = [
        "Build and send an HTTP request\n\n\n\n",
        "Download a remote file or directory\n\n\n\n",
        "View my stored API keys\n\n\n\n",
        "View or execute my saved commands\n\n\n\n",
    ];
    pub static ref REQUEST_MENU_OPTIONS: [&'static str; 8] = [
        "Add a URL\n\n\n\n",
        "Add Authentication\n\n\n\n",
        "Add Headers\n\n\n\n",
        "Enable verbose output\n\n\n\n",
        "Add Request Body\n\n\n\n",
        "Save this command\n\n\n\n",
        "Save your token or login\n\n\n\n",
        "Execute command\n\n\n\n",
    ];
    pub static ref DOWNLOAD_MENU_OPTIONS: [&'static str; 4] = [
        "Specify recursive depth\n\n\n\n",
        "Add a URL\n\n\n\n",
        "Specify output filepath\n\n\n\n",
        "Begin Download\n\n\n\n",
    ];
    pub static ref METHOD_MENU_OPTIONS: [&'static str; 5] = [
        "GET\n\n\n\n",
        "POST\n\n\n\n",
        "PUT\n\n\n\n",
        "DELETE\n\n\n\n",
        "PATCH\n\n\n\n",
    ];
    pub static ref AUTHENTICATION_MENU_OPTIONS: [&'static str; 8] = [
        "Basic\n\n\n\n",
        "Bearer\n\n\n\n",
        "Digest\n\n\n\n",
        "AWS SignatureV4\n\n\n\n",
        "Ntlm\n\n\n\n",
        "NtlmWb\n\n\n\n",
        "Kerberos\n\n\n\n",
        "SPNEGO\n\n\n\n",
    ];
    pub static ref RESPONSE_MENU_OPTIONS: [&'static str; 4] = [
        "Write to file?\n\n\n\n",
        "View response headers\n\n\n\n",
        "View response body\n\n\n\n",
        "Copy command to clipboard\n\n\n\n"
    ];
    pub static ref API_KEY_MENU_OPTIONS: [&'static str; 3] = [
        "Add a new key\n\n\n\n",
        "View my saved keys\n\n\n\n",
        "Delete a key\n\n\n\n",
    ];
    pub static ref DEBUG_MENU_OPTIONS: [&'static str; 2] =
        ["Back...\n \n", "URL Input Screen Debug \n \n"];
}
