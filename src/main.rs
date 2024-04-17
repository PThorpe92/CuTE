#![warn(clippy::all)]
#![allow(non_snake_case)]
use clap::{builder::Command, Arg};
use dirs::config_dir;
use std::io;
use std::sync::OnceLock;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use CuTE_tui::app::{App, AppResult};
use CuTE_tui::events::{
    event::{Event, EventHandler},
    handler::handle_key_events,
};
use CuTE_tui::{tui_cute::Tui, Config};

pub static CONFIG_PATH: OnceLock<String> = OnceLock::new();

fn main() -> AppResult<()> {
    let mut app = App::new();
    CONFIG_PATH.get_or_init(|| {
        config_dir()
            .unwrap()
            .join("CuTE/config.toml")
            .as_os_str()
            .to_string_lossy()
            .to_string()
    });
    app.set_config(parse_cmdline().unwrap_or_default());
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;
        if let Event::Key(key_event) = tui.events.next()? {
            handle_key_events(key_event, &mut app)?
        }
    }
    tui.exit()?;
    Ok(())
}

fn parse_cmdline() -> Option<Config> {
    let args = Command::new("CuTE")
        .author("PThorpe92 <preston@unlockedlabs.org>")
        .version("0.0.1")
        .about("Simple TUI for sending and storing HTTP requests, API keys and Postman collections")
        .after_help("Arguments are '--dump-config {path}' to write the default config file to the specified path,
            \nand '--db-path' to define a custom path to the database\nDB path can also be defined in the config file at $CONFIG/CuTE/config.toml\n
            or you can set the $CUTE_DB_PATH environment variable")
        .arg(
            Arg::new("db-path")
                .help("Define a custom path to the database")
                .id("db-path")
                .long("db-path"), // Added this line to indicate it takes a value
        )
        .arg(
            Arg::new("dump-config")
                .help("Write the default config file to the current working directory")
                .id("dump-config")
                .long("dump-config")
        ).get_matches();
    if args.contains_id("dump-config") {
        let mut config_path: String = args
            .get_one::<String>("dump-config")
            .expect("Missing dump-config argument")
            .to_string();
        if !config_path.contains("config.toml") {
            config_path.push_str("/config.toml");
        }
        let config = CuTE_tui::Config::default();
        let config = toml::to_string_pretty(&config).expect("Failed to serialize config");
        std::fs::write(config_path, config).expect("Failed to write config file");
    }
    if args.contains_id("db-path") {
        let db_path: String = args
            .get_one::<String>("db")
            .expect("Missing db-path argument")
            .to_string();
        let db_path = std::path::Path::new(&db_path);
        let db_path = std::fs::canonicalize(db_path).expect("Failed to canonicalize path");
        let mut config = Config::default();
        config.set_db_path(db_path);
        return Some(config);
    }
    None
}
