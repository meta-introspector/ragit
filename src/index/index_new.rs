use std::collections::HashMap;
use std::path::PathBuf;
use crate::constant::{API_CONFIG_FILE_NAME, BUILD_CONFIG_FILE_NAME, CHUNK_DIR_NAME, CONFIG_DIR_NAME, FILE_INDEX_DIR_NAME, II_DIR_NAME, IMAGE_DIR_NAME, INDEX_DIR_NAME, INDEX_FILE_NAME, LOG_DIR_NAME, MODEL_FILE_NAME, PROMPT_DIR_NAME, QUERY_CONFIG_FILE_NAME};
use crate::error::Error;
use ragit_fs::{create_dir_all, exists, into_abs_path, join, join3, normalize, write_bytes, write_string, WriteMode};
use serde_json::json;

use super::BuildConfig;
use crate::Path;
use crate::api_config::ApiConfig;
use crate::index::index_struct::Index;
use crate::prompts::PROMPTS;
use crate::QueryConfig;

impl Index {
    /// It works like git. `root_dir` is the root of the repo. And it creates dir `.ragit/`, like `.git/`.
    /// It reads the files in the repo and creates index.
    pub fn new(
    root_dir: PathBuf,
) -> Result<Self, Error> {
        let index_dir = join(&root_dir, INDEX_DIR_NAME)?;
        let root_dir = normalize(&into_abs_path(&root_dir)?)?;

        if exists(&index_dir) {
            return Err(Error::IndexAlreadyExists(index_dir));
        }

        create_dir_all(&index_dir)?;

        for dir in [
            CONFIG_DIR_NAME,
            CHUNK_DIR_NAME,
            IMAGE_DIR_NAME,
            FILE_INDEX_DIR_NAME,
            II_DIR_NAME,
        ] {
            create_dir_all(&Index::get_rag_path(
                &root_dir,
                &dir.to_string(),
            )?)?;
        }

        // Start with default configs
        let mut build_config = BuildConfig::default();
        let mut query_config = QueryConfig::default();
        let api_config = ApiConfig::default();

        // Create a temporary Index to use for loading configs from home
        let temp_index = Index {
            ragit_version: crate::VERSION.to_string(),
            chunk_count: 0,
            staged_files: vec![],
            processed_files: HashMap::new(),
            curr_processing_file: None,
            build_config: build_config.clone(),
            query_config: query_config.clone(),
            api_config: ApiConfig::default(),
            root_dir: root_dir.clone(),
            repo_url: None,
            ii_status: super::IIStatus::None,
            uid: None,
            summary: None,
            prompts: PROMPTS.clone(),
            models: vec![],
        };

        // Try to load build config from home directory and apply to defaults
        if let Ok(Some(partial_build_config)) = temp_index.load_build_config_from_home() {
            // Apply partial config to the default config
            partial_build_config.apply_to(&mut build_config);
        }

        // Try to load query config from home directory and apply to defaults
        if let Ok(Some(partial_query_config)) = temp_index.load_query_config_from_home() {
            // Apply partial config to the default config
            partial_query_config.apply_to(&mut query_config);
        }

        let mut result = Index {
            ragit_version: crate::VERSION.to_string(),
            chunk_count: 0,
            staged_files: vec![],
            processed_files: HashMap::new(),
            curr_processing_file: None,
            build_config,
            query_config,
            api_config,
            root_dir,
            repo_url: None,
            ii_status: super::IIStatus::None,
            uid: None,
            summary: None,
            prompts: PROMPTS.clone(),
            models: vec![],
        };

        // Load models first so we can choose an appropriate default model
        result.load_or_init_models()?;

        // Now update api_config with a valid model
        result.api_config = result.get_default_api_config()?;
        write_bytes(
            &result.get_build_config_path()?,
            &serde_json::to_vec_pretty(&result.build_config)?,
            WriteMode::AlwaysCreate,
        )?;
        write_bytes(
            &result.get_query_config_path()?,
            &serde_json::to_vec_pretty(&result.query_config)?,
            WriteMode::AlwaysCreate,
        )?;
        write_bytes(
            &result.get_api_config_path()?,
            &serde_json::to_vec_pretty(&result.api_config)?,
            WriteMode::AlwaysCreate,
        )?;
        result.save_to_file(result.root_dir.join(INDEX_FILE_NAME))?;

        Ok(result)
    }
}
