use crate::prelude::*;
use crate::index_struct::Index;

impl Index {
    pub fn load(root_dir: PathBuf, load_mode: LoadMode) -> Result<Self, ApiError> {
        if load_mode == LoadMode::Minimum {
            return Index::load_minimum(root_dir);
        }

        let mut result = Index::default();
        result.root_dir = root_dir;
        result.query_config = serde_json::from_str::<QueryConfig>(&read_string(
            &result.root_dir.join(CONFIG_DIR_NAME).join(QUERY_CONFIG_FILE_NAME).to_str().unwrap(),
        )?)?;
        result.api_config = serde_json::from_str::<ApiConfig>(&read_string(
            &result.root_dir.join(CONFIG_DIR_NAME).join(API_CONFIG_FILE_NAME).to_str().unwrap(),
        )?)?;

        if load_mode == LoadMode::QuickCheck && result.curr_processing_file.is_some() {
            return Ok(result);
        }

        if load_mode == LoadMode::Check {
            // TODO: check consistency
        }

        Ok(result)
    }
}
