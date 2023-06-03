use anyhow::{bail, Result};
use ratatui::backend::Backend;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use ratatui::Frame;

/// Completed will show a letter for each completed letter,
/// the 'letters' field represents each letter as a position in an array.
/// Since there are only 26 letters, we can represent them as a position in the
/// array, instead of using a HashMap.
pub struct Alphabet<'a> {
    title: &'a str,
    letters: [u32; 26],
}

impl<'a> Alphabet<'a> {
    /// Creates a new instance of Alphabet, with all letters set to 0u32
    pub fn new(title: &'a str) -> Self {
        Alphabet {
            title,
            letters: [0; 26],
        }
    }
    pub fn is_complete(&self) -> bool {
        !self.letters.iter().any(|&c| c == 0)
    }

    pub fn remove_letter(&mut self, c: &char) -> Result<()> {
        let a = *c as u32 - 65;
        if a > 25 {
            bail!("The character needs to be in the 'A'..='Z' range")
        }
        if self.letters[a as usize] >= 1 {
            self.letters[a as usize] -= 1;
        // } else if self.letters[a as usize] == 1 {
        //     self.letters[a as usize] -= 1;
        } else {
            // TODO: change the unreachable! macro into a better error handling
            bail!("You shouldn't be able to call 'remove_letter' on a letter that has a count of 0 in 'Alphabet.letters'");
        }
        Ok(())
    }

    pub fn add_letter(&mut self, c: &char) -> Result<()> {
        let a = *c as u32 - 65;
        // Make sure no unsafe array access
        if a > 25 {
            bail!("The character needs to be in the 'A'..='Z' range")
        }
        if self.letters[a as usize] == 0 {
            self.letters[a as usize] += 1;
        } else if self.letters[a as usize] == u32::MAX {
            bail!("You shouldn't be able to call 'add_letter' on a letter that has a count of u32::MAX in 'Alphabet.letters'");
        } else {
            self.letters[a as usize] += 1;
        }
        Ok(())
    }

    pub fn draw<B: Backend>(&self, frame: &mut Frame<B>, area: Rect, color: Color) {
        let s: String = self
            .letters
            .iter()
            .enumerate()
            .map(|(i, x)| if *x > 0 { (i as u8 + 65) as char } else { '.' })
            .collect();
        let text = Text::from(s);

        // let title = "Type to check for a pangram";
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(color))
            .border_type(BorderType::Rounded)
            .title(Span::styled(
                self.title,
                Style::default()
                    .fg(Color::White)
                    // .bg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ))
            .title_alignment(Alignment::Center);
        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        frame.render_widget(paragraph, area)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_must_use)]
    #[test]
    fn can_add_letter() {
        let mut a = Alphabet::new("test");
        a.add_letter(&'A');
        assert_eq!(a.letters[0], 1)
    }

    #[allow(unused_must_use)]
    #[test]
    fn can_remove_letter() {
        let mut a = Alphabet::new("test");
        a.add_letter(&'A');
        a.remove_letter(&'A');
        assert_eq!(a.letters[0], 0)
    }

    #[allow(unused_must_use)]
    #[test]
    fn can_detect_complete() {
        let mut a = Alphabet::new("test");
        for i in 'A'..='Z' {
            a.add_letter(&i);
        }
        assert!(a.is_complete())
    }

    #[allow(unused_must_use)]
    #[test]
    fn can_detect_complete_no_false_positive() {
        let mut a = Alphabet::new("test");
        for i in 'A'..='Y' {
            a.add_letter(&i);
        }
        assert!(!a.is_complete())
    }
}
