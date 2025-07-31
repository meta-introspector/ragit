
pub mod cli;
pub mod config;
pub mod error;
pub mod macros;
pub mod methods;
pub mod models;
pub mod utils;

// pub static CONFIG: OnceLock<crate::config::Config> = OnceLock::new();

pub fn dummy_server_lib() {
    panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
}

/*
use std::sync::OnceLock;

pub static CONFIG: OnceLock<crate::config::Config> = OnceLock::new();
*/