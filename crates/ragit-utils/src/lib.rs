pub mod prelude;

pub mod index;
pub mod error;
pub mod config;
pub mod api_config;
pub mod chunk;
pub mod constant;
pub mod prompts;
pub mod query;
pub mod agent;

pub use ragit_core_utils::string_utils;
pub mod uid;
pub mod cli_types;
pub mod api_utils;
pub mod project_root;
pub mod doc_utils;
pub mod ragit_path_utils;
pub use crate::ragit_path_utils as path_utils;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
