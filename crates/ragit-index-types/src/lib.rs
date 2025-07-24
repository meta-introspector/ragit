use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use ragit_types::uid::Uid;
// These imports will need to be resolved later, as they are not in ragit-types
// use ragit_types::api_error::ApiError;
// use ragit_types::model::Model;
// use ragit_types::query_config::QueryConfig;
// use ragit_types::api_config::ApiConfig;

// Placeholder for Model, QueryConfig, ApiConfig until their actual location is determined
pub struct Model;
pub struct QueryConfig;
pub struct ApiConfig;

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
