use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use ragit_types::uid::Uid;
use ragit_types::query::QueryConfig;
use ragit_types::api_config::ApiConfig;
use ragit_types::model::Model;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Index {
    pub root_dir: PathBuf,
    pub processed_files: HashMap<PathBuf, Uid>,
    pub staged_files: HashSet<PathBuf>,
    pub ragit_version: String,
    pub query_config: QueryConfig,
    pub api_config: ApiConfig,
    pub prompts: HashMap<String, String>,
    pub models: Vec<Model>,
    pub curr_processing_file: Option<PathBuf>,
    pub summary: Option<String>,
    pub uid: Uid,
}
