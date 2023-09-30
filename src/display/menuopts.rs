/*
* Screen Menu Options
*
 */

use lazy_static::lazy_static;

lazy_static! {
    pub static ref MAIN_MENU_OPTIONS: [&'static str; 4] = [
        "Build and send an HTTP request\n  \n",
        "Download a remote file or directory\n  \n",
        "View my stored API keys\n  \n",
        "View or execute my saved commands\n  \n",
    ];
    pub static ref REQUEST_MENU_OPTIONS: [&'static str; 8] = [
        "Add a URL\n \n",
        "Add Authentication\n \n",
        "Add Headers\n \n",
        "Enable verbose output\n \n",
        "Specify request output file\n \n",
        "Add Request Body\n \n",
        "Save this command\n \n",
        "Execute command\n \n",
    ];
    pub static ref DOWNLOAD_MENU_OPTIONS: [&'static str; 4] = [
        "Specify recursive depth\n \n",
        "Add a URL\n \n",
        "Specify output filepath\n \n",
        "Begin Download\n \n",
    ];
    pub static ref METHOD_MENU_OPTIONS: [&'static str; 5] = [
        "GET\n \n",
        "POST\n \n",
        "PUT\n \n",
        "DELETE\n \n",
        "PATCH\n \n",
    ];
    pub static ref AUTHENTICATION_MENU_OPTIONS: [&'static str; 8] = [
        "Basic\n \n",
        "Bearer\n \n",
        "Digest\n \n",
        "AWS SignatureV4\n \n",
        "Ntlm\n \n",
        "NtlmWb\n \n",
        "Kerberos\n \n",
        "SPNEGO\n \n",
    ];
    pub static ref INPUT_MENU_OPTIONS: [&'static str; 4] = [
        "Please enter a URL for your request",
        "Please specify your request headers",
        "Please enter your request body",
        "Please enter the filepath for"
    ];
    pub static ref RESPONSE_MENU_OPTIONS: [&'static str; 3] = [
        "Write to file?\n \n",
        "View response headers\n \n",
        "View response body\n \n",
    ];
    pub static ref API_KEY_MENU_OPTIONS: [&'static str; 3] = [
        "Add a new key\n \n",
        "View my saved keys\n \n",
        "Delete a key\n \n",
    ];
    pub static ref SAVED_COMMAND_OPTIONS: [&'static str; 3] = [
        "Add a new command\n \n",
        "View my saved commands\n \n",
        "Delete a saved command\n \n",
    ];
}
