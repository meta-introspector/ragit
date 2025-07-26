use std::path::PathBuf;
use ragit_types::api_config::ApiConfig;
use ragit_types::query::QueryConfig;
use ragit_types::uid::Uid;
use ragit_types::ii_status::IIStatus;
use ragit_types::summary::Summary;
use ragit_config::BuildConfig;

use ragit_api::Model;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


/// This is a knowledge-base itself. I am trying my best to define a method
/// for each command.
// NOTE: all the `Path` are normalized relative paths
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Index {
    pub ragit_version: String,
    pub chunk_count: usize,
    pub staged_files: Vec<PathBuf>,
    pub processed_files: HashMap<PathBuf, Uid>,

    /// Previously, all the builds were in serial and this field tells
    /// which file the index is building. When something goes wrong, ragit
    /// reads this field and clean up garbages. Now, all the builds are in
    /// parallel and there's no such thing like `curr_processing_file`. But
    /// we still need to tell whether something went wrong while building
    /// and this field does that. If it's `Some(_)`, something's wrong and
    /// clean-up has to be done.
    pub curr_processing_file: Option<PathBuf>,

    /// The name of this field has to be `remote`. It's my mistake.
    pub repo_url: Option<String>,

    /// `ii` stands for `inverted-index`.
    pub ii_status: IIStatus,

    pub uid: Option<Uid>,
    pub summary: Option<Summary>,

    #[serde(skip)]
    pub root_dir: PathBuf,
    #[serde(skip)]
    pub build_config: BuildConfig,
    #[serde(skip)]
    pub query_config: QueryConfig,
    #[serde(skip)]
    pub api_config: ApiConfig,
    #[serde(skip)]
    pub prompts: HashMap<String, String>,
    #[serde(skip)]
    pub models: Vec<Model>,
}

impl Index {
    pub fn new(root_dir: PathBuf) -> Self {
        Self {
            ragit_version: env!("CARGO_PKG_VERSION").to_string(),
            chunk_count: 0,
            staged_files: Vec::new(),
            processed_files: HashMap::new(),
            curr_processing_file: None,
            repo_url: None,
            ii_status: IIStatus::default(),
            uid: None,
            summary: None,
            root_dir,
            build_config: BuildConfig::default(),
            query_config: QueryConfig::default(),
            api_config: ApiConfig::default(),
            prompts: HashMap::new(),
            models: Vec::new(),
        }
    }
}
use crate::prelude::*;
use ragit_tfidf::load_from_file;
use ragit_error::ApiError;
use ragit_types::Chunk;

impl Index {
    pub async fn load_all_chunks(&self) -> Result<Vec<Chunk>, ApiError> {
        let mut chunks = vec![];

        for chunk_path in &ragit_index_chunk_methods::file_retrieval_methods::get_all_chunk_files(self)? {
            // Assuming load_from_file returns a Chunk, not ProcessedDoc
            chunks.push(ragit_tfidf::io::load_from_file(chunk_path.to_str().unwrap())?);
        }

        Ok(chunks)
    }
}