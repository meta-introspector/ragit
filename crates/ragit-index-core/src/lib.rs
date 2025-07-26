mod prelude;
pub use ragit_index_types::index_struct::Index;
use std::path::PathBuf;
use ragit_error::ApiError;
use serde_json;

pub fn load_index_from_path(path: &PathBuf) -> Result<Index, ApiError> {
    let index_json = std::fs::read_to_string(path)?;
    let index: Index = serde_json::from_str(&index_json)?;
    Ok(index)
}



























