![image](imgs/logo.gif)

![image](imgs/cute.png)
# Rust TUI HTTP Client with API Key Management

#### This project is still in active development and although it is definitely useable, there may still be bugs and significant changes are still needed to both refactor the codebase and add new features.
#### Collaboration is welcome and encouraged! There is lots of low hanging fruit 👍and cool ideas for additional features.
![image](imgs/demo.gif)

Terminal user interface (TUI) HTTP client in Rust designed to simplify the process of making various types of HTTP requests while supporting various different kinds of Authentication (powered by libcURL), recursive downloading of directories (powered by GNU Wget), and storage + management of your previous requests + API keys.

This tool is for when you don't need something as complex as Postman, but you also don't want to have to remember the syntax for `curl` (or `wget`) commands. 

## Features

- **Interactive TUI Interface**: The application offers an intuitive TUI interface that makes it easy to construct and execute HTTP requests without leaving the terminal.

- **Intuitive VIM keybindings:**  Vim keybindings are defaulted. Support to change them will eventually make it into the config file.

- **Multiple Request Types**: Support for GET, POST, PUT, PATCH, HEAD, DELETE and custom requests.

- **API Key Management**: Very simple sqlite based API key storage system. You can choose to save a Key from a request, or just add/edit/delete them manually.

- **Response Visualization**: Pretty-print JSON responses in a human-readable format within the TUI, or allows you to choose to write the response to a file. 

- **Cross Platform**: This application builds and runs on Linux, MacOS and even _Windows_. **Note** Recursive downloading is powered by `GNU Wget` (not the fake wget command you get on windows), so this functionality is only available through `Msys2` or `WSL` on Windows.


## Why?

- Have __you__ even ran `curl --help all` ?


## Installation

#### Prebuilt binaries for Windows and x86_64 Linux are available on the [Releases](https://github.com/PThorpe92/CuTE/tags) page.

### Install with Cargo:

- **Prerequisites**: Make sure you have Rust and Cargo installed on your system.

 1. `cargo install cute_tui`

 2. make sure that your `~/.cargo/bin` directory is in your PATH

 3. `cute` or `cute --dump-config .`  # this will put a config.toml file in your cwd. You can edit this and place it
                          in a dir `CuTE` in your `~/.config/` path (see below) to customize the colors of the application.


### Build from source:
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
5. **Move Binary**: Move the binary to a location in your PATH of your choosing:
   ```
   sudo cp target/release/cute /usr/local/bin
   ```

## Command Line Options

##### cute [OPTIONAL] '--dump-config <PATH>' or '--db-path <'/PATH/to/cute.db'>'

- **--dump-config**: Dumps the default config.toml file to the specified path. If no path is specified, it will output it to the current working directory.
  - This `config.toml` file needs to be placed in `~/.config/CuTE/{config.toml}` in order for the application to read it.
  - currently the config file can only specify basic colors of the application, and the path to the sqlite database. More options will be added in the future.

- **--db-path**: Specify the path to the sqlite database. If no path is specified, it will default to `data_local_dir` working directory.(~/.local/share/CuTE/CuTE.db or the windows/macos equivalent)

#### Menus

1. **Main Menu**: The main menu will provide options to create different types of HTTP requests and manage API keys.

2. **Request Type**: Select the type of HTTP request you would like to make. The tool supports GET, POST, PUT, PATCH, HEAD, DELETE and custom requests.

3. **API Key Management**: In the API key management section, you can add, edit, or delete API keys. Assign API keys to profiles and specific requests for easy integration.

4. **Viewing Responses**: After executing a request, the tool will display the response in a readable format within the TUI, with the option to write it out to a file.

5. **Saved Commands**: Much like the API keys, you can store and view past requests/commands for easy use later on.


## Contributing

Contributions to this project are welcome and encouraged! If you encounter any bugs, have suggestions for improvements, or want to add a new feature, feel free to open an issue or submit a PR.

Before contributing, please review the [Contribution Guidelines](CONTRIBUTING.md).


## License

This project is licensed under the [GPL3.0 License](LICENSE).

---
If you have any questions or need assistance, feel free to [reach out](p@eza.rocks)


## **Fun fact:**

>This project was developed in the Maine State Prison system, where the author is currently incarcerated. I would like to bring whatever awareness possible to the importance of education and rehabilitation for those 2.2 million Americans currently incarcerated. I have a [blog post](https://pthorpe92.github.io/intro/my-story/) if you are interested in reading about my story.


**Disclaimer:** This project is provided as-is, and its creators are not responsible for any misuse or potential security vulnerabilities resulting from the usage of API keys.
