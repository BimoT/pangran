use anyhow::{bail, Result};
use ratatui::backend::Backend;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use ratatui::Frame;

pub struct Inputbox<'a> {
    title: &'a str,
    cursor_position: u32,
    letters: String,
}

impl<'a> Inputbox<'a> {
    /// Constructs a new instance of Inputbox, with `cursor_position` = 0
    pub fn new(title: &'a str) -> Self {
        Inputbox {
            title,
            cursor_position: 0u32,
            letters: String::with_capacity(100),
        }
    }

    #[allow(dead_code)]
    pub fn get_letters(&self) -> &str {
        self.letters.as_ref()
    }

    #[allow(dead_code)]
    /// BASE FUNCTION, ONLY FOR TESTING PURPOSES
    /// use get_cursor() instead
    pub fn get_cursor_position(&self) -> u32 {
        self.cursor_position
    }

    // TODO: implement border_style checking
    // FIX: line breaking is based on words, not letters. This breaks the code.
    pub fn get_cursor(&self, area: Rect) -> (u16, u16) {
        let width = area.width - 2; // -2 because of single borders
        let base_x: u16 = self.cursor_position as u16;
        let x: u16;
        let y: u16;
        if width > base_x {
            x = base_x + 1;
            y = area.y + 1
            // (base_x, area.height as u32 + 1)
        } else if width == base_x {
            x = 0;
            y = area.y + 2;
            // (0, area.y as u32 + 2)
        } else {
            let rem = base_x % width;
            let div = base_x / width;
            x = area.x + rem + 1;
            y = area.y + div + 1;
        }

        (x, y)
    }

    /// Moves the cursor backwards by one position.
    /// If the cursor is already at the leftmost position, then it doesn't move.
    pub fn cursor_backward(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    /// Moves the cursor forwards by one position.
    /// Does nothing if the cursor position is u32::MAX
    pub fn cursor_forward(&mut self) {
        if self.cursor_position < self.letters.len() as u32 && self.cursor_position < u32::MAX {
            self.cursor_position += 1
        }
    }

    /// Does nothing if the text is u32::MAX,
    /// otherwise either pushes a character to the end of the letter string,
    /// or inserts it into the cursor position
    pub fn add_letter(&mut self, c: &char) -> Result<()> {
        let letterlen = self.letters.len() as u32;
        if letterlen == u32::MAX {
            bail!("You can type at most `{:?}` characters", u32::MAX);
        }

        if self.cursor_position == letterlen {
            self.letters.push(*c);
            self.cursor_position += 1;
        } else if self.cursor_position < letterlen {
            self.letters.insert(self.cursor_position as usize, *c);
            self.cursor_position += 1;
        } else {
            // TODO: decide how to handle the case where cursor_position > letters.len()
            self.letters.push(*c);
            self.cursor_position = self.letters.len() as u32;
        }
        Ok(())
    }

    /// Does nothing if there are no letters
    pub fn remove_letter(&mut self) -> Option<char> {
        let letterlen = self.letters.len() as u32;
        if letterlen == 0 {
            return None;
        }
        let removed_letter;
        if self.cursor_position == letterlen {
            removed_letter = self.letters.pop();
            self.cursor_backward();
        } else if self.cursor_position < letterlen {
            removed_letter = Some(self.letters.remove((self.cursor_position - 1) as usize));
            self.cursor_backward();
        } else {
            removed_letter = self.letters.pop();
            self.cursor_position = self.letters.len() as u32;
        }

        match removed_letter {
            Some(a) => {
                if ('A'..='Z').contains(&a) {
                    Some(a)
                } else if ('a'..='z').contains(&a) {
                    Some(a.to_ascii_uppercase())
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /// Should be called when 'delete' is pressed
    pub fn remove_next_letter(&mut self) -> Option<char> {
        self.cursor_forward();
        self.remove_letter()
    }

    /// Draws the inputbox in a specified area. The color of the borders can vary.
    pub fn draw<B: Backend>(&self, frame: &mut Frame<B>, area: Rect, color: Color) {
        let text = Text::from(self.letters.as_ref());
        let width = text.width();

        // let title = "Press 'ESC' to quit";
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
        let paragraph = Paragraph::new(text).block(block);
        // frame.set_cursor(area.x + self.get_cursor_position() as u16 + 1, area.y + 1);
        let (x, y) = self.get_cursor(area);
        frame.set_cursor(x, y);
        // frame.set_cursor(area.x + width as u16 + 1, area.y + 1);
        frame.render_widget(paragraph, area)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_must_use)]
    fn new() -> Inputbox<'static> {
        let mut inp = Inputbox::new("test");
        inp.add_letter(&'h');
        inp.add_letter(&'e');
        inp.add_letter(&'l');
        inp.add_letter(&'l');
        inp.add_letter(&'o');
        inp
    }
    #[test]
    fn can_add_multiple_letters() {
        let inp = new();
        assert_eq!(inp.letters, String::from("hello"))
    }

    #[allow(unused_must_use)]
    #[test]
    fn can_add_letters() {
        let mut inp = Inputbox::new("test");
        inp.add_letter(&'h');
        assert_eq!(inp.get_letters(), "h")
    }

    #[test]
    fn can_remove_letters() {
        let mut inp = new();
        inp.remove_letter();
        inp.remove_letter();
        assert_eq!(inp.get_letters(), "hel")
    }

    #[test]
    fn can_move_cursor() {
        let mut inp = new();
        inp.cursor_backward();
        inp.cursor_forward();
    }

    #[test]
    fn can_move_cursor_and_remove() {
        let mut inp = new();
        inp.cursor_backward();
        inp.cursor_backward();
        inp.cursor_backward();
        inp.remove_letter();
        assert_eq!(inp.get_letters(), "hllo")
    }

    #[allow(unused_must_use)]
    #[test]
    fn can_move_cursor_and_add() {
        let mut inp = new();
        inp.cursor_backward();
        inp.cursor_backward();
        inp.cursor_backward();
        inp.add_letter(&'Y');
        assert_eq!(inp.get_letters(), "heYllo")
    }

    #[test]
    fn can_delete_letters() {
        let mut inp = new();
        inp.cursor_backward();
        inp.remove_next_letter();
        assert_eq!(inp.get_letters(), "hell")
    }

    #[allow(unused_must_use)]
    #[test]
    fn can_add_non_ascii() {
        let mut inp = new();
        inp.add_letter(&'ü');
        assert_eq!(inp.get_letters(), "helloü")
    }
}
