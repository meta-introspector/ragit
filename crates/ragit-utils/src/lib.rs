pub mod prelude;

pub mod agent;
pub mod api_config;
pub mod chunk;
pub mod config;
pub mod constant;
pub mod error;
pub mod index;
pub mod prompts;
pub mod query;
pub mod string_utils;

pub mod api_utils;
pub mod cli_types;
pub mod doc_utils;
pub mod project_root;
pub mod ragit_path_utils;
pub mod uid;
pub use crate::ragit_path_utils as path_utils;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
