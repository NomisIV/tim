/// This file is responsible for handling the administrating part of the user interface. It
/// provides an interface for different views to access the terminal.
use std::io::Stdout;
use std::io::{stdin, stdout, Result, Write};
use std::sync::Mutex;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

use crate::imap_client::ImapClient;
use crate::structs::EmailHeader;

pub mod gui_list;
pub mod read_view;

use gui_list::GuiList;
use read_view::ReadView;

pub enum ActionResult {
    Update,
    NoUpdate,
    PushView(Box<dyn View>),
    PopView,
}

pub trait View {
    fn render(&self, buffer: &mut RawTerminal<Stdout>) -> Result<()>;
    fn handle_key(&mut self, key: Key) -> ActionResult;
}

pub fn ui(clients: &Vec<Mutex<ImapClient>>) -> Result<()> {
    // Get a list of headers for all emails from all inboxes
    // let mut emails: Vec<EmailHeader> = Vec::new();
    // for client in clients {
    //     let client_emails = client
    //         .lock()
    //         .unwrap()
    //         .list_mailbox("INBOX")
    //         .unwrap()
    //         .get_emails();
    //     emails.extend(client_emails)
    // }
    // let mut view: Box<dyn View> = Box::new(GuiList::new(emails));

    let mut view_stack: Vec<Box<dyn View>> = Vec::new();
    // view_stack.push(Box::new(ReadView::new("Hello World!")));
    view_stack.push(Box::new(GuiList::new(
        clients
            .get(0)
            .unwrap()
            .try_lock()
            .unwrap()
            .list_mailboxes()
            .unwrap(),
    )));

    // Grab the terminal
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    view_stack.last().unwrap().render(&mut stdout)?;

    // Main loop
    for key in stdin.keys() {
        // lol why does this work, rust?
        let key = key?;

        // Read input
        let result = match key {
            Key::Ctrl('c') => break,
            _ => view_stack.last_mut().unwrap().handle_key(key),
        };

        match result {
            ActionResult::Update => {}
            ActionResult::NoUpdate => continue,
            ActionResult::PushView(v) => {
                view_stack.push(v);
            }
            ActionResult::PopView => {
                view_stack.pop();
                if view_stack.is_empty() {
                    break;
                }
            }
        }
        view_stack.last().unwrap().render(&mut stdout)?;
    }

    write!(stdout, "{}", termion::cursor::Show)?;
    stdout.flush()?;

    Ok(())
}
