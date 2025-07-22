use ragit_uid::Uid;
use std::path::PathBuf;
use crate::api_config::ApiConfig;
use crate::error::Error;
use crate::query::QueryConfig;
use crate::prelude::*;
use ragit_api::Model;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


pub use super::config::BuildConfig;
pub use super::ii::IIStatus;

use crate::index::commands::summary::{Summary, SummaryMode};
use crate::index::commands::version::VersionInfo;



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
    pub fn get_summary(&self) -> Option<&str> {
        self.summary.as_ref().map(|s| s.as_str())
    }

    pub fn get_ragit_version_info(&self) -> VersionInfo {
        // Placeholder implementation
        VersionInfo {
            version: self.ragit_version.clone(),
            compatible: true, // Assume compatible for now
        }
    }

    pub fn migrate(&mut self, to_version: String) -> Result<(), Error> {
        // Placeholder implementation
        eprintln!("Migrating to version: {}", to_version);
        // In a real scenario, this would contain logic to migrate the index data
        // based on the `to_version`.
        Ok(())
    }

    pub async fn summary(&mut self, mode: Option<SummaryMode>) -> Result<Option<String>, Error> {
        // Placeholder implementation for summary generation
        eprintln!("Generating summary with mode: {:?}", mode);
        // In a real scenario, this would involve LLM calls to generate the summary.
        // For now, let's return a dummy summary.
        self.summary = Some(Summary(String::from("This is a generated summary.")));
        Ok(self.summary.as_ref().map(|s| s.0.clone()))
    }

    pub fn get_all_meta(&self) -> Result<HashMap<String, String>, Error> {
        let result = HashMap::new();

        Ok(result)
    }

    pub fn check_ii(&self) -> Result<(), Error> {
        // Placeholder implementation
        Ok(())
    }

    pub fn recover(&mut self) -> Result<(), Error> {
        // Placeholder implementation
        Ok(())
    }

    pub fn get_all_chunk_uids(&self) -> Result<Vec<Uid>, Error> {
        // Placeholder implementation
        Ok(vec![])
    }
}