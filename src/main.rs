use cURLtui_rs::app::{App, AppResult};
use cURLtui_rs::event::{Event, EventHandler};
use cURLtui_rs::handler::handle_key_events;
use cURLtui_rs::tui::Tui;
use std::io;
use std::process::Command;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> AppResult<()> {
    let mut app = App::new();
    if !is_command_available("curl") {
        eprintln!("Error: 'curl' is not installed on your system.");
        eprintln!("Please install 'curl' and try again.");
        std::process::exit(1);
    }

    // Check if 'wget' is installed
    if !is_command_available("wget") {
        eprintln!("Error: 'wget' is not installed on your system.");
        eprintln!("Please install 'wget' and try again.");
        std::process::exit(1);
    }

    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}

fn is_command_available(command: &str) -> bool {
    match Command::new("which").arg(command).status() {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}
