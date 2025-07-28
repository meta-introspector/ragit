use crate::index_struct::Index;
use ragit_types::ApiError;
use std::path::PathBuf;
use std::fs;
use std::io::Read;
use serde_json;
use crate::load_mode::LoadMode;
//use std::collections::HashMap;

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

        // Load prompts
        let prompts_dir = index.root_dir.join("prompts");
        if prompts_dir.exists() && prompts_dir.is_dir() {
            for entry in fs::read_dir(&prompts_dir).map_err(|e| {
                ApiError::Internal(format!(
                    "failed to read prompts directory: {}: {}",
                    prompts_dir.display(),
                    e
                ))
            })? {
                let entry = entry.map_err(|e| {
                    ApiError::Internal(format!("failed to read directory entry: {}", e))
                })?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "pdl" {
                            let mut file = fs::File::open(&path).map_err(|e| {
                                ApiError::Internal(format!(
                                    "failed to open prompt file: {}: {}",
                                    path.display(),
                                    e
                                ))
                            })?;
                            let mut prompt_content = String::new();
                            file.read_to_string(&mut prompt_content).map_err(|e| {
                                ApiError::Internal(format!(
                                    "failed to read prompt file: {}: {}",
                                    path.display(),
                                    e
                                ))
                            })?;
                            if let Some(file_stem) = path.file_stem() {
                                if let Some(file_stem_str) = file_stem.to_str() {
                                    index.prompts.insert(file_stem_str.to_string(), prompt_content);
                                }
                            }
                        }
                    }
                }
            }
        }

        // TODO: Load configs from their respective files
        // index.build_config = ...
        // index.query_config = ...
        // index.api_config = ...
        // index.models = ...

        Ok(index)
    }
}
