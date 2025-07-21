pub mod prelude;
pub mod uid;
pub mod index;
pub mod error;
pub mod config;
pub mod api_config;
pub mod chunk;
pub mod constant;
pub mod prompts;
pub mod query;
pub mod path_utils;
pub mod agent;
pub mod string_utils;

pub use path_utils::{pathbuf_to_str, str_to_pathbuf, join_paths, join3_paths, get_rag_path, path_to_display, str_to_path_ref};

pub use uid::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
