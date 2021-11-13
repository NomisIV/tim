use std::io::Result;
use std::io::Stdout;
use std::io::Write;
use std::cmp::min;

use termion::event::Key;
use termion::raw::RawTerminal;

use super::ActionResult;
use super::View;

pub trait Listable {
    fn render(&self, buffer: &mut RawTerminal<Stdout>) -> Result<()>;
    fn select(&self) -> Box<dyn View>;
}

pub struct GuiList<T: Listable> {
    index: usize,
    // TODO: This means that every mail will be it's own object, which is rather inefficient
    list: Vec<T>,
}

impl<T: Listable> GuiList<T> {
    pub fn new(list: Vec<T>) -> Self {
        GuiList { list, index: 0 }
    }
}

impl<T: Listable> View for GuiList<T> {
    fn render(&self, buffer: &mut RawTerminal<Stdout>) -> Result<()> {
        write!(buffer, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1))?;

        let rows = min(self.list.len(), (termion::terminal_size().unwrap().1 - 5).into());
        for n in 0..rows {
            if n == self.index {
                write!(buffer, "> ")?;
                self.list.get(n).unwrap().render(buffer)?;
            } else {
                write!(buffer, "  ")?;
                self.list.get(n).unwrap().render(buffer)?;
            }
            write!(buffer, "\r\n")?;
        }

        Ok(())
    }

    fn handle_key(&mut self, key: Key) -> ActionResult {
        match key {
            Key::Up => {
                if self.index > 0 {
                    self.index -= 1;
                    ActionResult::Update
                } else {
                    ActionResult::NoUpdate
                }
            }
            Key::Down => {
                if self.index + 1 < self.list.len() {
                    self.index += 1;
                    ActionResult::Update
                } else {
                    ActionResult::NoUpdate
                }
            }
            Key::Char('o') | Key::Char('\n') => {
                let view = self.list.get(self.index).unwrap().select();
                ActionResult::PushView(view)
            }
            Key::Char('q') | Key::Esc => ActionResult::PopView,
            _ => ActionResult::NoUpdate,
        }
    }
}
