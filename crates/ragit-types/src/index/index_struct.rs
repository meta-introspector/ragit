use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
//use crate::api_error::ApiError;
//use crate::model::Model;
//use crate::query_config::QueryConfig;
use crate::uid::Uid;
//use crate::api_config::ApiConfig;
//use ragit_utils::config::*;
use crate::query::QueryConfig;
use crate::api_config::ApiConfig;
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
