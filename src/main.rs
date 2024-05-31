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
use ratatui::style::{Color, Style};
use ratatui::widgets::Paragraph;
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

const HELP: &str = r". Any character    ^ Start of string    $ End of string    * 0+ occurrences       + 1+ occurrences     ? 0 or 1 occurrence
 {n} Exactly n      {n,} n+ occurrences  {,m} At most m     {n,m} n to m occ.      [abc] Any of a,b,c   [^abc] None of a,b,c
 [a-z] a to z       [A-Z] A to Z         [0-9] Any digit    \d Any digit           \D Non-digit         \w Word character
 \W Non-word char   \s Any whitespace    \S Non-whitespace  \t Tab                 \n Newline           \r Carriage return
 \\ Backslash       \. Literal dot       \+ Literal plus    \* Literal asterisk    \? Literal question  | Alternation
 (...) Grouping     (?=...) Lookahead    (?!...) Neg lookahead (?<=...) Lookbehind  (?<!...) Neg lookbehind
 i Case-insensitive g Global search      m Multi-line mode  s Dot matches newline

";

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

    let banner = Paragraph::new("Regect")
        .right_aligned()
        .style(Style::default().fg(Color::Cyan));
    let help = Paragraph::new(HELP);
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(7),
        ]);

    loop {
        term.draw(|f| {
            let chunks = layout.split(f.size());
            f.render_widget(banner.clone(), chunks[0]);
            f.render_widget(help.clone(), chunks[3]);
            f.render_widget(regex_input.textarea.widget(), chunks[1]);
            match mode {
                Mode::RegexEdit => f.render_widget(body.highlighted_body(), chunks[2]),
                Mode::BodyEdit => f.render_widget(body.textarea.widget(), chunks[2]),
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
