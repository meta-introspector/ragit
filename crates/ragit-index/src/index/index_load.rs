use crate::prelude::*;

pub use crate::index::load_mode::LoadMode;

impl Index {
    pub fn load(root_dir: PathBuf, load_mode: LoadMode) -> Result<Self, ApiError> {
        if load_mode == LoadMode::Minimum {
            return Index::load_minimum(root_dir);
        }

        let mut result = Index::default();
        result.root_dir = root_dir;
        result.query_config = serde_json::from_str::<QueryConfig>(&read_string(
            &result.root_dir.join(CONFIG_DIR_NAME).join(QUERY_CONFIG_FILE_NAME),
        )?)?;
        result.api_config = serde_json::from_str::<ApiConfig>(&read_string(
            &result.root_dir.join(CONFIG_DIR_NAME).join(API_CONFIG_FILE_NAME),
        )?)?;

        result.load_or_init_models()?;
        result.load_or_init_prompts()?;

        if load_mode == LoadMode::QuickCheck && result.curr_processing_file.is_some() {
            return Ok(result);
        }

        if load_mode == LoadMode::Check {
            // TODO: check consistency
        }

        Ok(result)
    }

    pub fn load_minimum(root_dir: PathBuf) -> Result<Self, ApiError> {
        let mut result = Index::default();
        result.root_dir = get_normalized_abs_pathbuf(&root_dir)?;

        result.uid = if exists(&get_rag_path(&root_dir, &PathBuf::from(INDEX_FILE_NAME))?) {
            read_string(&get_rag_path(&root_dir, &PathBuf::from(INDEX_FILE_NAME))?)?.parse()?;
        } else {
            Uid::new();
        };

        result.query_config = serde_json::from_str::<QueryConfig>(&read_string(
            &result.root_dir.join(CONFIG_DIR_NAME).join(QUERY_CONFIG_FILE_NAME),
        )?)?;
        result.api_config = serde_json::from_str::<ApiConfig>(&read_string(
            &result.root_dir.join(CONFIG_DIR_NAME).join(API_CONFIG_FILE_NAME),
        )?)?;

        Ok(result)
    }
}