use crate::app::App;
use crate::display::displayopts::DisplayOpts;
use crate::request::cmdtype::CmdType;
use crate::request::curl::AuthKind;
use crate::screens::Screen;
use crate::{app::InputMode, display::inputopt::InputOpt};
use tui::prelude::Line;
use tui::style::Color;
use tui::widgets::Paragraph;
use tui::widgets::{Block, Borders};
use tui::{
    prelude::{Backend, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Text},
    Frame,
};

/// Renders a screen we can grab input from, pass in the appropriate designation for the input
pub fn get_input_prompt(opt: InputOpt) -> Text<'static> {
    return match opt {
        InputOpt::URL(opt) => {
            let fmtstr = format!("Enter a URL for your {}\n and press Enter", opt);
            Text::from(Line::from(fmtstr))
        }
        InputOpt::Headers => Text::from(Line::from(
            "MUST be \"Key:Value\" pair and press Enter \n Example: Content-Type: application/json",
        )),
        InputOpt::RecursiveDownload => {
            Text::from("Enter the recursion level and press Enter \n Example: 2")
        }
        _ => Text::from("Enter a value and press Enter"),
    };
}

pub fn handle_default_input_screen<B: Backend>(
    app: &mut App,
    frame: &mut Frame<'_, B>,
    opt: InputOpt,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(frame.size());
    let (_msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to submit."),
            ],
            Style::default(),
        ),
    };
    let mut prompt = get_input_prompt(opt.clone());
    prompt.patch_style(style);
    render_input_with_prompt(frame, prompt);

    let width = chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor

    let scroll = app.input.visual_scroll(width as usize);
    let input = Paragraph::new(app.input.value())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::LightBlue),
        })
        .scroll((0, scroll as u16))
        .block(Block::default().borders(Borders::ALL).title("Input"));
    frame.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => frame.set_cursor(
            chunks[1].x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            chunks[1].y + 1,
        ),
    }

    if !app.messages.is_empty() {
        app.input_mode = InputMode::Normal;
        parse_input(app.messages[0].clone(), opt, app);
        app.messages.remove(0);
    }
}

fn parse_input(message: String, opt: InputOpt, app: &mut App) {
    match opt {
        InputOpt::URL(opt) => {
            app.set_url(message.clone());
            match opt {
                CmdType::Wget => {
                    app.add_display_option(DisplayOpts::URL(message));
                    app.goto_screen(Screen::Downloads);
                }
                CmdType::Curl => {
                    app.add_display_option(DisplayOpts::URL(message));
                    app.goto_screen(Screen::RequestMenu(String::new()));
                }
            };
        }
        InputOpt::RequestBody => {
            // This means they should be pasting in a large request body...
            //  not sure how to handle this yet.
        }
        InputOpt::Verbose => {
            // This shouldn't have sent them to this screen...
        }
        InputOpt::Headers => {
            let headers = message.split(':').collect::<Vec<&str>>();
            let cpy = (
                String::from(headers[0].clone()),
                String::from(headers[1].clone()),
            );
            app.command
                .as_mut()
                .unwrap()
                .set_headers(headers.iter().map(|x| x.to_string()).collect());
            app.add_display_option(DisplayOpts::Headers(cpy));
            app.current_screen = Screen::RequestMenu(String::new());
        }

        InputOpt::Output => {
            app.input_mode = InputMode::Normal;
            app.command.as_mut().unwrap().set_outfile(&message);
            app.add_display_option(DisplayOpts::Outfile(message.clone()));
            app.goto_screen(Screen::RequestMenu(String::new()));
        }
        InputOpt::Execute => {
            // This means they have executed the command, and want to write to a file
            app.input_mode = InputMode::Normal;
            app.command.as_mut().unwrap().set_outfile(&message);
            match app.command.as_mut().unwrap().write_output() {
                Ok(_) => {
                    app.goto_screen(Screen::Success);
                }
                Err(e) => {
                    app.goto_screen(Screen::Error(e.to_string()));
                }
            }
        }
        InputOpt::RecursiveDownload => {
            let recursion_level = message.parse::<usize>().unwrap();
            app.command
                .as_mut()
                .unwrap()
                .set_rec_download_level(recursion_level);
            app.add_display_option(DisplayOpts::RecDownload(recursion_level));
            app.goto_screen(Screen::Downloads);
        }
        InputOpt::Auth(auth) => {
            parse_auth(auth, app, message);
        }
    }
}

fn render_input_with_prompt<B: Backend>(frame: &mut Frame<'_, B>, prompt: Text) {
    // Render the input with the provided prompt
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let message = Paragraph::new(prompt);
    frame.render_widget(message, chunks[0]);
}

fn parse_auth(auth: AuthKind, app: &mut App, message: String) {
    match auth {
        AuthKind::Basic(_) => {
            app.command
                .as_mut()
                .unwrap()
                .set_auth(AuthKind::Basic(message.clone()));
            app.add_display_option(DisplayOpts::Auth(message));
        }

        AuthKind::Bearer(_) => {
            app.command
                .as_mut()
                .unwrap()
                .set_auth(AuthKind::Bearer(message.clone()));
            app.add_display_option(DisplayOpts::Auth(message));
        }
        AuthKind::Digest(_) => {
            app.command
                .as_mut()
                .unwrap()
                .set_auth(AuthKind::Digest(message.clone()));
            app.add_display_option(DisplayOpts::Auth(message));
        }
        AuthKind::AwsSigv4(_) => {
            app.command
                .as_mut()
                .unwrap()
                .set_auth(AuthKind::AwsSigv4(message.clone()));
            app.add_display_option(DisplayOpts::Auth(message));
        }
        AuthKind::Spnego(_) => {
            app.command
                .as_mut()
                .unwrap()
                .set_auth(AuthKind::Spnego(message.clone()));
            app.add_display_option(DisplayOpts::Auth(message));
        }
        AuthKind::Kerberos(_) => {
            app.command
                .as_mut()
                .unwrap()
                .set_auth(AuthKind::Kerberos(message.clone()));
            app.add_display_option(DisplayOpts::Auth(message));
        }
        AuthKind::Ntlm(_) => {
            app.command
                .as_mut()
                .unwrap()
                .set_auth(AuthKind::Ntlm(message.clone()));
            app.add_display_option(DisplayOpts::Auth(message));
        }
        AuthKind::NtlmWb(_) => {
            app.command
                .as_mut()
                .unwrap()
                .set_auth(AuthKind::NtlmWb(message.clone()));
            app.add_display_option(DisplayOpts::Auth(message));
        }
        AuthKind::None => {}
    };
}
