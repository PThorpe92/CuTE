/*
* Screen Menu Options
 */
use lazy_static::lazy_static;

lazy_static! {

    pub static ref MAIN_MENU_OPTIONS: [&'static str; 4] = [
        "Build and send an HTTP request\n  \n  \n  \n",
        "Download a remote file or directory\n \n \n  \n",
        "View my stored API keys\n  \n  \n   \n",
        "View or execute my saved commands\n \n \n  \n",
    ];
    pub static ref REQUEST_MENU_OPTIONS: [&'static str; 7] = [
        "Add a URL\n \n",
        "Add Authentication\n \n",
        "Add Headers\n \n",
        "Enable verbose output\n \n",
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
    pub static ref RESPONSE_MENU_OPTIONS: [&'static str; 3] = [
        "Write to file?\n \n \n ",
        "View response headers\n \n \n",
        "View response body\n \n \n",
    ];
    pub static ref API_KEY_MENU_OPTIONS: [&'static str; 3] = [
        "Add a new key\n \n \n ",
        "View my saved keys\n \n \n",
        "Delete a key\n \n \n",
    ];
    pub static ref DEBUG_MENU_OPTIONS: [&'static str; 2] =
        ["Back...\n \n", "URL Input Screen Debug \n \n"];

    pub static ref SAVED_COMMANDS_PARAGRAPH: &'static str = "View / Delete my saved cURL commands.\nPress q to exit\nPress Enter to select\nPress h to go back\n Please select a Menu item\n";
    pub static ref SAVED_COMMANDS_TITLE: &'static str = "My Saved cURL Commands";
    pub static ref CURL: &'static str = "curl";
    pub static ref WGET: &'static str = "wget";
    pub static ref CUSTOM: &'static str = "custom";
    pub static ref METHOD_MENU_TITLE: &'static str = "Choose a Method";
    pub static ref DOWNLOAD: &'static str = "Download";
    pub static ref API_KEY_PARAGRAPH: &'static str = "Create / Edit / Delete API Keys and tokens.\nPress q to exit\nPress Enter to select\n Please select a Menu item\n";
    pub static ref API_KEY_TITLE: &'static str = "My API Keys";
    pub static ref HTTP_REQUEST: &'static str = "HTTP Request";
    pub static ref HOME_MENU_PARAGRAPH: &'static str =
        "\nPress q to exit \n Press Enter to select \n Please select a Menu item\n";
    pub static ref HOME_MENU_TITLE: &'static str = "* CuTE *";
    pub static ref AUTH_MENU_TITLE: &'static str = "Authentication Menu";
    pub static ref DOWNLOAD_MENU_TITLE: &'static str = "* CuTE ** Downloads *";
    pub static ref SUCCESS_TITLE: &'static str = "* CuTE *\n* Success! *";
    pub static ref SUCCESS_MESSAGE: &'static str = "Command saved successfully";
}
