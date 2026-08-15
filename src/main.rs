use std::io::{self, BufWriter, IsTerminal};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        self,
        cursor::Show,
        event::{DisableMouseCapture, EnableMouseCapture},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
mod banners;
mod body;
mod regex_input;

struct TerminalGuard;

impl TerminalGuard {
    fn init(out: &mut impl io::Write) -> io::Result<Self> {
        enable_raw_mode()?;
        crossterm::execute!(out, EnterAlternateScreen, EnableMouseCapture)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = crossterm::execute!(
            io::stderr(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            Show
        );
    }
}

fn read_from_stdin() -> Option<Vec<String>> {
    let input = io::stdin();
    if !input.is_terminal() {
        Some(input.lines().map_while(Result::ok).collect())
    } else {
        None
    }
}

fn main() -> io::Result<()> {
    let input = read_from_stdin();
    let output = io::stderr();
    let mut output = output.lock();

    let _guard = TerminalGuard::init(&mut output)?;
    let mut term = Terminal::new(CrosstermBackend::new(BufWriter::new(output)))?;

    let mut app = app::App::new(input);
    let output = app.run(&mut term)?;

    if let Some(output) = output {
        println!("{}", output);
    }

    Ok(())
}
