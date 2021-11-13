use std::path::PathBuf;
use std::process::exit;
use std::sync::Mutex;

use structopt::StructOpt;

mod cfg;
mod imap_client;
mod structs;
mod ui;

use cfg::*;
use imap_client::ImapClient;

// TODO: I could probably get away with using clap only
/// Terminal Interface eMail client
#[derive(StructOpt)]
pub struct Opt {
    /// Location of configuration file
    #[structopt(short, long)]
    pub config: Option<PathBuf>,

    /// Use this account only for all operations
    #[structopt(short, long)]
    pub account: Option<String>,
}

fn main() {
    // Load the CLI arguments
    let opt = Opt::from_args();

    // Load the config file
    let cfg_path = get_cfg_path(opt.config).unwrap();
    let cfg = cfg::load(cfg_path);
    if let Err(err) = cfg {
        eprintln!("{}", err);
        exit(1);
    }
    let cfg = cfg.unwrap();

    // Login to the selected accounts
    let accounts: Vec<Mutex<ImapClient>> = if let Some(a) = opt.account {
        // If one account is specified, use only it
        vec![Mutex::new(
            ImapClient::new(cfg.get_account(&a).unwrap()).unwrap(),
        )]
    } else {
        // Else, use all of them
        cfg.get_all_accounts()
            .iter()
            .map(|acc| Mutex::new(ImapClient::new(acc).unwrap()))
            .collect()
    };

    // Start UI
    let ui = ui::ui(&accounts);
    if let Err(err) = ui {
        eprintln!("{}", err);
        exit(1);
    }

    // Log out
    for acc in accounts {
        acc.lock().unwrap().logout();
    }
}
