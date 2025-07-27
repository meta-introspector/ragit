use crate::index_struct::Index;
use ragit_types::ApiError;
use std::path::PathBuf;
use std::fs;
use serde_json;
use crate::load_mode::LoadMode;

impl Index {
    pub fn load(path: PathBuf, _mode: LoadMode) -> Result<Self, ApiError> {
        let index_path = path.join(".ragit").join("index.json");
        if !index_path.exists() {
            return Err(ApiError::Internal(format!(
                "index file not found: {}",
                index_path.display()
            )));
        }

        let content = fs::read_to_string(&index_path).map_err(|e| {
            ApiError::Internal(format!(
                "failed to read index file: {}: {}",
                index_path.display(),
                e
            ))
        })?;

        let mut index: Index = serde_json::from_str(&content).map_err(|e| {
            ApiError::Internal(format!(
                "failed to parse index file: {}: {}",
                index_path.display(),
                e
            ))
        })?;

        index.root_dir = path;
        // TODO: Load configs from their respective files
        // index.build_config = ...
        // index.query_config = ...
        // index.api_config = ...
        // index.prompts = ...
        // index.models = ...

        Ok(index)
    }
}
