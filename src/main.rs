use std::io;

use body::{banner, captures, footer, help, TestInput};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};
use regex_input::RegexInput;
use tui_textarea::{Input, Key};

mod body;
mod regex_input;
#[derive(Clone)]
enum EditMode {
    RegexEdit,
    BodyEdit,
}

impl EditMode {
    fn toggle(&self) -> Self {
        match self {
            EditMode::RegexEdit => EditMode::BodyEdit,
            EditMode::BodyEdit => EditMode::RegexEdit,
        }
    }
}

enum InfoMode {
    QuickReference,
    Captures,
}

impl InfoMode {
    fn toggle(&self) -> Self {
        match self {
            InfoMode::QuickReference => InfoMode::Captures,
            InfoMode::Captures => InfoMode::QuickReference,
        }
    }
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut mode = EditMode::RegexEdit;
    let mut info_mode = InfoMode::Captures;
    let mut regex_input = RegexInput::new();
    let mut body = TestInput::new();

    let mut result = String::new();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]);

    loop {
        term.draw(|f| {
            let chunks = layout.split(f.size());
            f.render_widget(banner(), chunks[0]);
            f.render_widget(footer(), chunks[4]);

            match mode {
                EditMode::RegexEdit => {
                    f.render_widget(regex_input.textarea.widget(), chunks[1]);
                    f.render_widget(
                        body.highlighted_body(regex_input.current_regex()),
                        chunks[2],
                    );
                }
                EditMode::BodyEdit => {
                    f.render_widget(regex_input.unfocused(), chunks[1]);
                    f.render_widget(body.textarea.widget(), chunks[2]);
                }
            }

            match info_mode {
                InfoMode::QuickReference => f.render_widget(help(), chunks[3]),
                InfoMode::Captures => f.render_widget(captures(), chunks[3]),
            }
        })?;

        match (crossterm::event::read()?.into(), mode.clone()) {
            (
                Input {
                    key: Key::Char('q'),
                    ctrl: true,
                    ..
                },
                _,
            ) => break,
            (
                Input {
                    key: Key::Enter, ..
                },
                EditMode::RegexEdit,
            ) => {
                result = regex_input.textarea.lines()[0].clone();
                break;
            }
            (
                Input {
                    key: Key::Char('x'),
                    ctrl: true,
                    ..
                },
                _,
            ) => mode = mode.toggle(),
            (Input { key: Key::Esc, .. }, _) => mode = mode.toggle(),
            (
                Input {
                    key: Key::Char('h'),
                    ctrl: true,
                    ..
                },
                _,
            ) => info_mode = info_mode.toggle(),
            (input, EditMode::BodyEdit) => {
                body.textarea.input(input);
            }
            (input, _) => {
                if regex_input.textarea.input(input) {
                    regex_input.validate()
                }
            }
        }
    }

    disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;
    println!("{}", result);

    Ok(())
}
