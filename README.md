![demo](https://github.com/PThorpe92/curl-tui-rs/assets/121899304/e50b009d-e766-48c8-9c6b-c1bbe07d00d2)



# Rust TUI HTTP Client with API Key Management


Terminal user interface (TUI) HTTP client in Rust designed to simplify the process of making various types of HTTP requests while supporting various different kinds of Authentication (powered by libcURL), recursive downloading of files (powered by WGET), and storage + management of your API keys.

Have you ever wanted to just grab some data from an API, or demonstrate that your REST endpoint is working, and had to craft some 2 paragraph long curl CLI command, just to forget every flag and option you used the next time you need to do send the same command? 

Now, not only can you execute these commands from a simple TUI and either view the response or write it to a file, but on top of executing the command with libcurl, _CuTE_ will build the actual curl command string you need to run it again, should you wish to share it with someone else, use it on a server, or just keep it stashed for later.

## Features

- **Interactive TUI Interface**: The application offers an intuitive TUI interface that makes it easy to construct and execute HTTP requests without leaving the terminal.

- **Intuitive VIM keybindings:**  Navigate the TUI using the familiar Vim keybindings you know and love.

- **Multiple Request Types**: With this tool, you can effortlessly create and send all the standard HTTP request types, and even use multiple forms of Authentication, without knowing an entire sub-language known as `curl-cli-flags`. This ensures flexibility in your interaction with different APIs.

- **API Key Management**: The project includes a simple-to-use API key management system. You can store your API keys within the application and assign them names for easy reference.

- **Response Visualization**: The tool pretty-prints JSON and XML responses in a human-readable format within the TUI. This enables quick assessment of the results of your requests.

- **Cross Platform**: This application builds and runs on Linux, MacOS and even _Windows_.


## Why?

- Have __you__ even ran `curl --help all` ?

## Why don't you support `X`?

- See above `why`: Supporting every available action or authentication type in libcurl would be a monumental task. If there are enough requests for a specific feature, it will be considered. Otherwise, PR's are welcome.


## Installation

1. **Prerequisites**: Make sure you have Rust and Cargo installed on your system.

2. **Clone the Repository**: Clone this repository to your local machine using the following command:
   ```
   git clone https://github.com/PThorpe92/CuTE.git
   ```

3. **Navigate to Project Directory**: Move into the project directory:
   ```
   cd CuTE
   ```

4. **Build and Run**: Build and run the application using Cargo:
   ```
   cargo build --release 
   ```

## Usage

Upon launching the application, you'll be presented with the TUI interface. Here's a quick guide to using the features:

1. **Main Menu**: The main menu will provide options to create different types of HTTP requests and manage API keys.

2. **Request Type**: Select the type of HTTP request you would like to make. The tool supports GET, POST, PUT, PATCH, and DELETE requests.

3. **API Key Management**: In the API key management section, you can add, edit, or delete API keys. Assign API keys to profiles and specific requests for easy integration.

4. **Viewing Responses**: After executing a request, the tool will display the response in a readable format within the TUI, with the option to write it out to a file.

5. **Saved Commands**: Much like the API keys, you can store and view past requests/commands for easy use later on.

## Contributing

Contributions to this project are welcome! If you encounter any bugs, have suggestions for improvements, or want to add new features, feel free to open an issue or submit a pull request.

Before contributing, please review the [Contribution Guidelines](CONTRIBUTING.md).

## License

This project is licensed under the [GPL3.0 License](LICENSE).

---

Happy coding! If you have any questions or need assistance, feel free to reach out to [Preston T](https://github.com/PThorpe92)

**Disclaimer:** This project is provided as-is, and its creators are not responsible for any misuse or potential security vulnerabilities resulting from the usage of API keys.
