mod body;
mod input;
use std::io;

use body::Body;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use input::RegexInput;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::Paragraph,
    Terminal,
};
use tui_textarea::{Input, Key};

#[derive(Clone)]
enum Mode {
    RegexEdit,
    BodyEdit,
    QuickReference,
}

const BANNER: &str = r"┏┓┏┓┏┓┏┓┏╋
┛ ┗ ┗┫┗ ┗┗
";

const FOOTER: &str = r"^x to switch input, ^h quick reference, ^q quit";

impl Mode {
    fn toggle(&self) -> Self {
        match self {
            Mode::RegexEdit => Mode::BodyEdit,
            Mode::BodyEdit => Mode::RegexEdit,
            Mode::QuickReference => Mode::RegexEdit,
        }
    }

    fn toggle_reference(&self) -> Self {
        match self {
            Mode::QuickReference => Mode::RegexEdit,
            _ => Mode::QuickReference,
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

    let mut mode = Mode::RegexEdit;
    let mut regex_input = RegexInput::new();
    let mut body = Body::new();

    let mut result = String::new();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]);

    loop {
        term.draw(|f| {
            let chunks = layout.split(f.size());
            let banner = Paragraph::new(BANNER)
                .centered()
                .style(Style::default().fg(Color::Cyan));
            let footer = Paragraph::new(FOOTER).right_aligned();
            f.render_widget(banner, chunks[0]);
            f.render_widget(footer, chunks[3]);

            match mode {
                Mode::RegexEdit => {
                    f.render_widget(regex_input.textarea.widget(), chunks[1]);
                    f.render_widget(
                        body.highlighted_body(regex_input.current_regex()),
                        chunks[2],
                    );
                }
                Mode::QuickReference => {
                    f.render_widget(regex_input.textarea.widget(), chunks[1]);
                    f.render_widget(body.help(), chunks[2]);
                }
                Mode::BodyEdit => {
                    f.render_widget(regex_input.unfocused(), chunks[1]);
                    f.render_widget(body.textarea.widget(), chunks[2]);
                }
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
                Mode::RegexEdit,
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
            ) => mode = mode.toggle_reference(),
            (input, Mode::BodyEdit) => {
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
