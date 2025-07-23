use super::{BuildConfig, IIStatus};
use crate::index::index_struct::Index;
use crate::{api_config::ApiConfig, query::config::QueryConfig};
use std::collections::HashMap;
use std::path::PathBuf;

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
}
