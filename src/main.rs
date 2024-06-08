use std::io;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod app;
mod banners;
mod body;
mod regex_input;

fn read_from_stdin() -> Option<Vec<String>> {
    if !atty::is(atty::Stream::Stdin) {
        Some(io::stdin().lines().map(|l| l.unwrap()).collect())
    } else {
        None
    }
}

fn main() -> io::Result<()> {
    let input = read_from_stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut app = app::App::new(input);
    let output = app.run(&mut term)?;

    disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;

    if let Some(output) = output {
        println!("{}", output);
    }

    Ok(())
}
