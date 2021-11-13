use std::io::Result;
use std::io::Stdout;
use std::io::Write;

use termion::event::Key;
use termion::raw::RawTerminal;

use crate::ui::View;

use super::ActionResult;

pub struct ReadView {
    content: String,
}

impl ReadView {
    pub fn new(content: &str) -> Self {
        ReadView {
            content: content.to_string(),
        }
    }
}

impl View for ReadView {
    fn render(&self, buffer: &mut RawTerminal<Stdout>) -> Result<()> {
        write!(buffer, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1))?;
        write!(buffer, "{}\r\n", self.content)?;

        Ok(())
    }

    fn handle_key(&mut self, key: Key) -> ActionResult {
        match key {
            Key::Char('q') | Key::Esc => ActionResult::PopView,
            _ => ActionResult::NoUpdate,
        }
    }
}
