use crate::prelude::*;
use crate::index_struct::Index;
use ragit_utils::ragit_path_utils::{get_normalized_abs_pathbuf, join_paths, get_rag_path};
use ragit_fs::{exists, create_dir_all};
use ragit_error::ApiError;
use ragit_types::{QueryConfig, ApiConfig, Uid};
use ragit_types::model::Model;
use crate::index_struct::save_to_file::save_to_file;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use lazy_static::lazy_static;
use crate::{VERSION, PROMPTS, CONFIG_DIR_NAME, CHUNK_DIR_NAME, IMAGE_DIR_NAME, FILE_INDEX_DIR_NAME, II_DIR_NAME, INDEX_FILE_NAME};

impl Index {
    pub fn new(root_dir: PathBuf) -> Result<Self, ApiError> {
        let root_dir = get_normalized_abs_pathbuf(&root_dir)?;
        let index_dir = join_paths(&root_dir, &PathBuf::from(INDEX_DIR_NAME))?;

        if exists(&index_dir) {
            return Err(ApiError::IndexExists(index_dir));
        }

        create_dir_all(index_dir.to_str().unwrap())?;

        for dir in [
            CONFIG_DIR_NAME,
            CHUNK_DIR_NAME,
            IMAGE_DIR_NAME,
            FILE_INDEX_DIR_NAME,
            II_DIR_NAME,
        ] {
            create_dir_all(get_rag_path(&root_dir, &PathBuf::from(dir))?.to_str().unwrap())?;
        }

        let query_config = QueryConfig::default();
        let api_config = ApiConfig::default();

        let mut result = Index {
            root_dir,
            processed_files: HashMap::new(),
            staged_files: HashSet::new(),
            ragit_version: VERSION.to_string(),
            query_config,
            api_config,
            prompts: PROMPTS.clone(),
            models: Default::default(),
            curr_processing_file: None,
            summary: None,
            uid: Uid::dummy(),
        };

        ragit_index_save_to_file::save_index_to_file(&result, result.root_dir.join(INDEX_FILE_NAME))?;

        Ok(result)
    }
}
