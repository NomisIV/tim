# nec - Next Email Client

## Usage examples

## Test

```console
$ nec [list|ls]
<account> <mailbox> (<unread>/<total>)
test@nomisiv.com INBOX  (1/1)
test@nomisiv.com Drafts (0/0)
test@nomisiv.com Sent   (0/0)
...

$ nec list INBOX
<num>: [<tags>] <date> <time> <sender>: <subject>: <body>
1: [N] Mon 12 Aug 14:37 test@nomisiv.com: Hello tere! 

$ nec read INBOX 1
Sender: <sender>
Subject: <subject>
Date: <date> <time>

<body>

$ nec read
list_select(mailbox)
list_select(mail)

# Lists all the unread mesasges from all accounts and all mailboxes
$ nec unread

$ nec reply [mailbox] [index]

// Reply to last read message
$ nec reply --last
```

This sticks:

- `nec` - Shows list of mails from all inboxes combined
<!-- - `nec unread` - Shows list of all unread messages in inboxes combined -->
- `nec write [--to <address>] [--subject <subject>] [-cc <cc>] [-bcc <bcc>] [--attach <file>] [--body <body>]` - Writes a new email

## General thoughts

A CLI generally assumes the user knows exactly what they want to do with each command.
Maybe it would be smarter to keep things more imperative, like a cli?
What about a REPL shell, with commands like cd, ls and cat?

`nec` should ask for more information instead of exiting when possible.

A way to specify a specific mail would be really nice

What about this:
`$ nec` by default shows a list of all the mailboxes for all accounts.
Selecting a mailbox reveals it's contents,
and when a mail is selected,
the user can chose what to do from the mail-specific options
(like, read, delete, reply, move, flag etc.)

Then the only other commands needed would be nec unread for a list of
unread mails, and nec write for writing new email.

