use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Color;
use ratatui::{Frame, Terminal};

use crate::alphabet::Alphabet;
use crate::input::Inputbox;

pub struct App<'a> {
    alphabet: Alphabet<'a>,
    // alphabet: [char; 26],
    inputbox: Inputbox<'a>,
    // already_typed: String,
    should_quit: bool,
    // empty_alphabet: [char; 26],
}

impl<'a> App<'a> {
    pub fn new(alphabet_title: &'a str, inputbox_title: &'a str) -> App<'a> {
        App {
            // TODO: deprecate title field, or put it in the Alphabet field
            alphabet: Alphabet::new(alphabet_title),
            inputbox: Inputbox::new(inputbox_title),
            // already_typed: String::with_capacity(100),
            should_quit: false,
        }
    }

    fn should_quit(&mut self) {
        self.should_quit = true
    }

    fn on_char(&mut self, c: char) -> Result<()> {
        match c {
            ('a'..='z') => {
                self.inputbox.add_letter(&c)?;
                let letter = c.to_ascii_uppercase();
                self.alphabet.add_letter(&letter)?;
            }
            ('A'..='Z') => {
                self.inputbox.add_letter(&c)?;
                self.alphabet.add_letter(&c)?;
            }
            // '!' => {
            //     self.should_quit = true;
            // }
            _ => {
                self.inputbox.add_letter(&c)?;
            }
        }
        Ok(())
    }

    pub fn draw<B: Backend>(&self, frame: &mut Frame<B>, color: Color) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
            .split(frame.size());
        self.alphabet.draw(frame, chunks[0], color);
        self.inputbox.draw(frame, chunks[1], color);
    }

    pub fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        let mut color = Color::Red;

        loop {
            terminal.draw(|f| self.draw(f, color))?;

            match crossterm::event::read()? {
                // TODO: test if ctrl-c is handled correctly
                Event::Key(key) if key.modifiers == KeyModifiers::CONTROL => {
                    if key.code == KeyCode::Char('c') {
                        self.should_quit = true
                    }
                }
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char(c) => {
                        self.on_char(c)?;
                    }
                    KeyCode::Esc => {
                        self.should_quit();
                    }
                    KeyCode::Backspace => {
                        let letter = self.inputbox.remove_letter();
                        if let Some(c) = letter {
                            self.alphabet.remove_letter(&c)?;
                        }
                    }
                    KeyCode::Delete => {
                        let letter = self.inputbox.remove_next_letter();
                        if let Some(c) = letter {
                            self.alphabet.remove_letter(&c)?;
                        }
                    }
                    KeyCode::Left => {
                        self.inputbox.cursor_backward();
                    }
                    KeyCode::Right => {
                        self.inputbox.cursor_forward();
                    }
                    _ => continue,
                },
                Event::Resize(_, _) => continue,
                _ => continue,
            }
            // TODO: handle color change if alphabet is complete
            if self.alphabet.is_complete() {
                color = Color::Green
            } else {
                color = Color::Red
            }
            if self.should_quit {
                return Ok(());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_results)]
    #[test]
    fn can_quit() {
        let mut a = App::new("test", "test");
        a.should_quit();
        assert!(a.should_quit)
    }

    #[allow(unused_must_use)]
    #[test]
    fn can_add_chars_without_quitting() {
        let mut a = App::new("test", "test");
        a.on_char('t');
        a.on_char('e');
        a.on_char('s');
        a.on_char('t');
        assert!(!a.should_quit)
    }

    #[allow(unused_must_use)]
    #[test]
    fn can_add_chars() {
        let mut a = App::new("test", "test");
        a.on_char('t');
        a.on_char('e');
        a.on_char('s');
        a.on_char('t');
        assert_eq!(a.inputbox.get_letters(), "test")
    }

    #[allow(unused_must_use)]
    #[test]
    fn can_move_its_own_cursor() {
        let mut a = App::new("test", "test");
        a.on_char('t');
        a.on_char('e');
        a.on_char('s');
        a.on_char('t');
        assert_eq!(a.inputbox.get_cursor_position(), 4)
    }
}
