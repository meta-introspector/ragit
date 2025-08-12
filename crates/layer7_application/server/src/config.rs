// use crate::error::Error;
// use ragit_fs::read_string;
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Config {
//     // If set, it dumps log to this file.
//     pub log_file: Option<String>,

//     // If set, it dumps log to stdout.
//     // `log` and `dump_log_to_stdout` are independent to each other.
//     pub dump_log_to_stdout: bool,

//     // A directory where push sessions are stored.
//     pub push_session_dir: String,
//     pub repo_data_dir: String,
//     pub blob_dir: String,
//     pub only_admin_can_create_user: bool,
//     pub port_number: u16,
// }

pub struct Config;

impl Config {
    pub fn load_from_file(_file: &str) -> Result<Config, anyhow::Error> {
        panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
    }
}

impl Default for Config {
    fn default() -> Config {
        panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
    }
}

/*
use crate::error::Error;
use ragit_fs::read_string;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    // If set, it dumps log to this file.
    pub log_file: Option<String>,

    // If set, it dumps log to stdout.
    // `log` and `dump_log_to_stdout` are independent to each other.
    pub dump_log_to_stdout: bool,

    // A directory where push sessions are stored.
    pub push_session_dir: String,
    pub repo_data_dir: String,
    pub blob_dir: String,
    pub only_admin_can_create_user: bool,
    pub port_number: u16,
}

impl Config {
    pub fn load_from_file(file: &str) -> Result<Config, Error> {
        let j = read_string(file)?;
        Ok(serde_json::from_str(&j)?)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            log_file: Some(String::from("./ragit-server-logs")),
            dump_log_to_stdout: false,
            push_session_dir: String::from("./session"),
            repo_data_dir: String::from("./data"),
            blob_dir: String::from("./blobs"),
            only_admin_can_create_user: true,
            port_number: 41127,
        }
    }
}
*/