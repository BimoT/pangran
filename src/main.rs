use std::io;

use anyhow::Result;
use crossterm::cursor::{EnableBlinking, Show};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{self, execute};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

mod alphabet;
mod app;
mod input;

use crate::app::App;

fn run() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        Show,
        // SetCursorStyle::DefaultUserShape,
        EnableBlinking,
    )?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new("Press 'ESC' to quit", "Start typing to check for a pangram");
    terminal.show_cursor()?;
    let res = app.run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        // SetCursorStyle::DefaultUserShape,
    )?;
    terminal.show_cursor()?;
    if let Err(err) = res {
        eprintln!("{err:?}");
    }

    Ok(())
}

const VERSION: &str = "pangran 0.0.1";
const INFO: &str = r#"
pangran: an interactive pangram checker.

usage: pangram [options]

options:
    -h, --help    Print help information
    -v, --version Print version information

A pangram is a series of words (ideally a sentence) that contains every letter in the alphabet. The most famous pangram is "The quick brown fox jumps over the lazy dog", but there are many more.
Run the program without any arguments to start the TUI, where you can type and check if you have written a pangram. Pressing the 'Escape' key quits the TUI. Holding the 'Control' key and pressing the letter 'c' also quits the TUI.
"#;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if let Some(a) = args.get(1) {
        match a.as_str() {
            "-h" | "--help" => println!("{}", INFO),
            "-v" | "--version" => println!("{}", VERSION),
            _ => println!("{}", INFO),
        }
    } else {
        run()?
    }
    Ok(())
}
