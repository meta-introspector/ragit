use ragit_config::BuildConfig;
use ragit_utils::api_config::ApiConfig;
use ragit_utils::query::QueryConfig;
use std::collections::HashMap;
use std::path::PathBuf;

use super::{Index, IIStatus};

impl Index {
    pub fn dummy() -> Self {
        Index {
            ragit_version: String::new(),
            chunk_count: 0,
            staged_files: vec![],
            processed_files: HashMap::new(),
            curr_processing_file: None,
            repo_url: None,
            ii_status: IIStatus::None,
            uid: None,
            summary: None,
            root_dir: PathBuf::from("."),
            build_config: BuildConfig::default(),
            query_config: QueryConfig::default(),
            api_config: ApiConfig::default(),
            prompts: HashMap::new(),
            models: vec![],
        }
    }

    pub fn dummy_with_version(version: String) -> Self {
        let mut dummy_index = Index::dummy();
        dummy_index.ragit_version = version;
        dummy_index
    }
}