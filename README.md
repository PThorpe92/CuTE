![image](https://github.com/PThorpe92/curl-tui-rs/test/examples/demo.gif)

# Rust TUI HTTP Client with API Key Management


Terminal user interface (TUI) HTTP client in Rust designed to simplify the process of making various types of HTTP requests, including `curl`, `wget`, and custom requests. Additionally, the tool
provides convenient functionality for managing and storing API keys securely. No more having to remember endless cURL flags and craft extensive commands.

## Features

- **Interactive TUI Interface**: The application offers an intuitive TUI interface that makes it easy to construct and execute HTTP requests without leaving the terminal.

- **Multiple Request Types**: With this tool, you can effortlessly create and send `curl`, `wget`, and custom HTTP requests. This ensures flexibility in your interaction with different APIs.

- **API Key Management**: The project includes a secure API key management system. You can store your API keys within the application and easily associate them with specific requests.

- **Configurable Profiles**: Configure different profiles for various projects or APIs, each with its own set of stored API keys and preferences.

- **Response Visualization**: The tool displays the HTTP responses in a human-readable format within the TUI. This enables quick assessment of the results of your requests.

- **History and Favorites**: Keep track of your recent requests in a history log, and mark specific requests as favorites for quicker access.

## Installation

1. **Prerequisites**: Make sure you have Rust and Cargo installed on your system.

2. **Clone the Repository**: Clone this repository to your local machine using the following command:
   ```
   git clone https://github.com/PThorpe92/curl-tui-rs.git
   ```

3. **Navigate to Project Directory**: Move into the project directory:
   ```
   cd curl-tui-rs
   ```

4. **Build and Run**: Build and run the application using Cargo:
   ```
   cargo run
   ```

## Usage

Upon launching the application, you'll be presented with the TUI interface. Here's a quick guide to using the features:

1. **Main Menu**: The main menu will provide options to create different types of HTTP requests and manage API keys.

2. **Creating Requests**: Select the desired request type (e.g., `curl`, `wget`, or custom). Follow the prompts to input the necessary parameters such as URL, headers, and request body.

3. **API Key Management**: In the API key management section, you can add, edit, or delete API keys. Assign API keys to profiles and specific requests for easy integration.

4. **Viewing Responses**: After executing a request, the tool will display the response in a readable format within the TUI.

5. **History and Favorites**: Use the history log to revisit recent requests and mark essential requests as favorites.

## Contributing

Contributions to this project are welcome! If you encounter any bugs, have suggestions for improvements, or want to add new features, feel free to open an issue or submit a pull request.

Before contributing, please review the [Contribution Guidelines](CONTRIBUTING.md).

## License

This project is licensed under the [GPL3.0 License](LICENSE).

---

Happy coding! If you have any questions or need assistance, feel free to reach out to me, [Preston T](https://github.com/PThorpe92)

**Disclaimer:** This project is provided as-is, and its creators are not responsible for any misuse or potential security vulnerabilities resulting from the usage of API keys.
