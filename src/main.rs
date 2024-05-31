mod body;
mod input;
use std::io;

use body::Body;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use input::RegexInput;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Terminal;
use tui_textarea::{Input, Key};

enum Mode {
    RegexEdit,
    BodyEdit,
}

impl Mode {
    fn toggle(&self) -> Self {
        match self {
            Mode::RegexEdit => Mode::BodyEdit,
            Mode::BodyEdit => Mode::RegexEdit,
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
    let mut regex_input = RegexInput::new();
    let mut body = Body::new();

    let mut mode = Mode::RegexEdit;

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(3), Constraint::Fill(1)]);

    loop {
        term.draw(|f| {
            let chunks = layout.split(f.size());
            let widget = regex_input.textarea.widget();
            f.render_widget(widget, chunks[0]);
            match mode {
                Mode::RegexEdit => f.render_widget(body.highlighted_body(), chunks[1]),
                Mode::BodyEdit => f.render_widget(body.textarea.widget(), chunks[1]),
            }
        })?;

        match crossterm::event::read()?.into() {
            Input { key: Key::Esc, .. } => break,
            Input { key: Key::Tab, .. } => mode = mode.toggle(),
            input => match mode {
                Mode::RegexEdit => {
                    if regex_input.textarea.input(input) {
                        regex_input.validate()
                    }
                }
                Mode::BodyEdit => {
                    body.textarea.input(input);
                }
            },
        }
    }

    disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;

    Ok(())
}
