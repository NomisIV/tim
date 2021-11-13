use std::sync::Arc;
use std::sync::Mutex;

use imap::Result;

use crate::cfg::Account;
use crate::structs::{Email, Mailbox, MailboxTitle};

type ImapSession = Arc<Mutex<imap::Session<native_tls::TlsStream<std::net::TcpStream>>>>;

#[derive(Debug, Clone)]
pub struct ImapClient {
    session: ImapSession,
}

impl ImapClient {
    pub fn new(acc: &Account) -> Result<ImapClient> {
        let tls = native_tls::TlsConnector::new()?;
        let session = imap::connect(
            (acc.imap.host.to_string(), acc.imap.port.unwrap_or(993)),
            &acc.imap.host,
            &tls,
        )?
        .login(&acc.email, &acc.get_password().unwrap())
        .map_err(|e| e.0)?;

        Ok(ImapClient {
            session: Arc::new(Mutex::new(session)),
        })
    }

    pub fn logout(&self) {
        self.session.lock().unwrap().logout().unwrap();
    }

    pub fn read_email(&self, mailbox: &str, index: usize) -> Result<Email> {
        self.session.lock().unwrap().examine(mailbox)?;

        let raw_email = self
            .session
            .lock()
            .unwrap()
            .fetch(index.to_string(), "RFC822")?;
        let email = raw_email.get(0).map(|email| email.body().unwrap()).unwrap();

        Ok(Email::new(email))
    }

    pub fn list_mailbox(&self, mailbox: &str) -> Result<Mailbox> {
        self.session.lock().unwrap().examine(mailbox)?;

        let raw_headers = self.session.lock().unwrap().fetch("1:*", "BODY[HEADER]")?;
        let headers: Vec<&[u8]> = raw_headers
            .iter()
            .map(|header| header.header().unwrap())
            .collect();

        Ok(Mailbox::new(mailbox, headers, self.clone()))
    }

    pub fn list_mailboxes(&self) -> Result<Vec<MailboxTitle>> {
        // TODO: Add more info about each mailbox, like the number of unread emails

        let dirs = self
            .session
            .lock()
            .unwrap()
            .list(None, Some("*"))?
            .iter()
            .map(|dir| MailboxTitle::new(dir.name(), self.clone()))
            .collect();

        Ok(dirs)
    }
}
