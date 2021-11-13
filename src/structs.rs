use std::io::Result;
use std::io::Stdout;
use std::io::Write;

use termion::raw::RawTerminal;
use mailparse::*;

use crate::ui::read_view::ReadView;
use crate::ui::gui_list::GuiList;
use crate::ui::View;
use crate::imap_client::ImapClient;
use crate::ui::gui_list::Listable;

#[derive(Debug)]
pub struct Email {
    // header: EmailHeader,
    body: String,
}

impl Email {
    pub fn new(email: &[u8]) -> Email {
        let parsed_email = mailparse::parse_mail(email).unwrap();
        Email {
            // header,
            body: parsed_email.get_body().unwrap(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct EmailHeader {
    mailbox: String,
    index: usize,
    subject: String,
    client: ImapClient,
}

impl EmailHeader {
    pub fn new(mailbox: &str, index: usize, header: &[u8], client: ImapClient) -> EmailHeader {
        let (headers, _) = mailparse::parse_headers(header).unwrap();
        EmailHeader {
            mailbox: mailbox.to_string(),
            index,
            subject: headers
                .get_first_value("Subject")
                .unwrap_or("Failed to read subject".to_string()),
                client,
        }
    }
}

impl Listable for EmailHeader {
    fn render(&self, buffer: &mut RawTerminal<Stdout>) -> Result<()> {
        write!(buffer, "{}", self.subject)?;
        Ok(())
    }

    fn select(&self) -> Box<dyn View> {
        Box::new(ReadView::new(&self.client.read_email(&self.mailbox, self.index).unwrap().body))
    }
}

#[derive(Clone)]
pub struct Mailbox {
    name: String,
    mails: Vec<EmailHeader>,
    client: ImapClient,
}

impl Mailbox {
    pub fn new(name: &str, headers: Vec<&[u8]>, client: ImapClient) -> Mailbox {
        // let mails = headers
        //     .iter()
        //     .map(|header| EmailHeader::new(header, client))
        //     .collect();

        let mut mails = Vec::new();
        for index in 0..headers.len() {
            mails.push(EmailHeader::new(name, index + 1, headers.get(index).unwrap(), client.clone()))
        }

        Mailbox {
            name: name.to_string(),
            mails,
            client,
        }
    }

    pub fn get_emails(&self) -> Vec<EmailHeader> {
        self.mails.clone()
    }
}

pub struct MailboxTitle {
    title: String,
    client: ImapClient,
}

impl MailboxTitle {
    pub fn new(title: &str, client: ImapClient) -> Self {
        Self {
            title: title.to_string(),
            client,
        }
    }
}

impl Listable for MailboxTitle {
    fn render(&self, buffer: &mut RawTerminal<Stdout>) -> Result<()> {
        write!(buffer, "{}", self.title)?;
        Ok(())
    }

    fn select(&self) -> Box<dyn View> {
        Box::new(GuiList::new(self.client.list_mailbox(&self.title).unwrap().get_emails()))
    }
}
