/// This file is responsible for reading, parsing, and allowing for easy access to the
/// configuration data.
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde::Deserialize;

#[derive(Debug)]
pub struct CfgError {
    message: String,
}

impl CfgError {
    fn new(message: &str) -> Self {
        CfgError {
            message: message.to_string(),
        }
    }
}

impl Error for CfgError {}

impl fmt::Display for CfgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Deserialize)]
pub struct Cfg {
    accounts: HashMap<String, Account>,
    // email_header_format: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Account {
    pub name: String,
    pub email: String,
    password: Option<String>,
    pass_cmd: Option<String>,
    pub signature: Option<String>,
    pub imap: Server,
    pub smtp: Server,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub host: String,
    pub port: Option<u16>,
}

impl Cfg {
    pub fn get_account(&self, name: &str) -> Option<&Account> {
        self.accounts
            .iter()
            .find(|(n, _a)| &name == n)
            .map(|(_n, a)| a)
    }

    pub fn get_all_accounts(&self) -> Vec<&Account> {
        self.accounts.iter().map(|(_, v)| v).collect()
    }
}

impl Account {
    pub fn get_password(&self) -> Result<String, CfgError> {
        if let Some(pass) = &self.password {
            Ok(pass.to_owned())
        } else if let Some(pass_cmd) = &self.pass_cmd {
            let mut cmd = pass_cmd.split(" ");

            let output = Command::new(&cmd.next().unwrap())
                .args(cmd.collect::<Vec<&str>>())
                .output()
                .map_err(|err| {
                    CfgError::new(&format!(
                        "Failed to get password for user \"{}\": {}",
                        self.email, err
                    ))
                })?;

            if output.status.code().unwrap() > 0 {
                return Err(CfgError::new(&format!(
                    "Failed to get password for user \"{}\": {}",
                    self.email,
                    String::from_utf8(output.stderr).unwrap()
                )));
            }

            let password = std::str::from_utf8(&output.stdout)
                .unwrap()
                .lines()
                .next()
                .unwrap()
                .trim_end()
                .to_string();

            Ok(password)
        } else {
            Err(CfgError::new("No password to authenticate with!"))
        }
    }
}

pub fn load(path: PathBuf) -> Result<Cfg, CfgError> {
    let config_raw = fs::read(path)
        .map_err(|err| CfgError::new(&format!("Could not read configuration file: {}", err)))?;

    let config = toml::from_slice(&config_raw)
        .map_err(|err| CfgError::new(&format!("Could not parse configuration file: {}", err)))?;

    Ok(config)
}

pub fn get_cfg_path(path_maybe: Option<PathBuf>) -> Option<PathBuf> {
    path_maybe.or_else(|| {
        if let Some(base_dirs) = directories::BaseDirs::new() {
            let mut config_dir = base_dirs.config_dir().to_path_buf();
            config_dir.push("tim");
            config_dir.push("config.toml");
            Some(config_dir)
        } else {
            None
        }
    })
}
