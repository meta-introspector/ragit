use crate::prelude::*;
use ragit_types::api_config::ApiConfig;
impl Index {
    pub fn new(root_dir: PathBuf) -> Result<Self, ApiError> {
        let root_dir = get_normalized_abs_pathbuf(&root_dir)?;
        let index_dir = join_paths(&root_dir, INDEX_DIR_NAME)?;

        if exists(&index_dir) {
            return Err(ApiError::IndexExists(index_dir));
        }

        create_dir_all(&index_dir)?;

        for dir in [
            CONFIG_DIR_NAME,
            CHUNK_DIR_NAME,
            IMAGE_DIR_NAME,
            FILE_INDEX_DIR_NAME,
            II_DIR_NAME,
        ] {
            create_dir_all(&get_rag_path(&root_dir.to_path_buf(), dir)?)?;
        }

        let mut query_config = QueryConfig::default();
        let api_config = ApiConfig::default();

        let mut result = Index {
            root_dir,
            processed_files: HashMap::new(),
            staged_files: HashSet::new(),
            ragit_version: VERSION.to_string(),
            query_config,
            api_config,
            prompts: PROMPTS.clone(),
            models: Model::default_models(),
            curr_processing_file: None,
            summary: None,
            uid: Uid::new(),
        };

        result.save_to_file(result.root_dir.join(INDEX_FILE_NAME).to_str().unwrap())?;

        Ok(result)
    }
}
