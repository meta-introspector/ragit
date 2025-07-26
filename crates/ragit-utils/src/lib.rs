pub mod prelude;
pub mod constant;
pub mod error;
pub mod prompts;
pub mod query;
pub mod string_utils;
pub mod api_utils;
pub mod cli_types;
pub mod doc_utils;
pub mod project_root;
pub mod ragit_path_utils;
pub mod index;
pub mod chunk;
pub mod path_utils;
pub mod uid_helpers;
pub use uid_helpers::uid_new_file;

pub mod version_info;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");