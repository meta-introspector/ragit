pub mod add_files;
pub use add_files::add_files_command;
pub mod prelude;
use ragit_error::ApiError;
use ragit_index_types::index_struct::Index;
use std::path::PathBuf;

use serde_json;
use std::fs;

pub fn load_index_from_path(path: &PathBuf) -> Result<Index, ApiError> {
    let index_json = fs::read_to_string(path)?;
    let index: Index = serde_json::from_str(&index_json)?;
    Ok(index)
}