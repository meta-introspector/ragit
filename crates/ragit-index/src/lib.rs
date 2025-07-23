use ragit_utils::api_config::ApiConfig;
use ragit_utils::error::Error;
use ragit_utils::prelude::*;
use ragit_utils::query::QueryConfig;
use ragit_api::Model;
use ragit_uid::Uid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// From ragit-utils/src/index
pub use ragit_utils::index::config::BuildConfig;
pub use ragit_utils::index::ii::IIStatus;

// From ragit-commands
use ragit_commands::summary::{Summary, SummaryMode};
use ragit_commands::version::VersionInfo;
use ragit_commands::merge::MergeMode;
use ragit_commands::remove::RemoveResult;
use ragit_commands::add::{AddMode, AddResult};
use ragit_commands::pull::PullResult;
use ragit_commands::push::PushResult;

// From ragit-types
use ragit_types::{FileSchema, ImageSchema, ChunkSchema, ProcessedDoc};

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

pub mod agent_methods;

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

    pub async fn merge(
        &mut self,
        base: String,
        prefix: Option<String>,
        merge_mode: MergeMode,
        quiet: bool,
        dry_run: bool,
    ) -> Result<(), Error> {
        eprintln!(
            "Placeholder for merge: base={}, prefix={:?}, merge_mode={:?}, quiet={}, dry_run={}",
            base, prefix, merge_mode, quiet, dry_run
        );
        Ok(())
    }

    pub async fn search_remote_models(
        keyword: &str,
        remote: &str,
    ) -> Result<Vec<ragit_api::Model>, Error> {
        eprintln!(
            "Placeholder for search_remote_models: keyword={}, remote={}",
            keyword, remote
        );
        Ok(vec![])
    }

    pub async fn fetch_remote_models(
        &mut self,
        model_name: &str,
        existing_only: bool,
        remote: &str,
    ) -> Result<ragit_api::FetchResult, Error> {
        eprintln!(
            "Placeholder for fetch_remote_models: model_name={}, existing_only={}, remote={}",
            model_name, existing_only, remote
        );
        Ok(ragit_api::FetchResult {
            fetched: 0,
            updated: 0,
        })
    }

    pub async fn fetch_all_remote_models(
        &mut self,
        existing_only: bool,
        remote: &str,
    ) -> Result<ragit_api::FetchResult, Error> {
        eprintln!(
            "Placeholder for fetch_all_remote_models: existing_only={}, remote={}",
            existing_only, remote
        );
        Ok(ragit_api::FetchResult {
            fetched: 0,
            updated: 0,
        })
    }

    pub fn remove_local_model(&mut self, model_name: &str) -> Result<(), Error> {
        eprintln!(
            "Placeholder for remove_local_model: model_name={}",
            model_name
        );
        Ok(())
    }

    pub fn remove_all_local_models(&mut self) -> Result<(), Error> {
        eprintln!("Placeholder for remove_all_local_models");
        Ok(())
    }

    pub fn gc_logs(&mut self) -> Result<usize, Error> {
        eprintln!("Placeholder for gc_logs");
        Ok(0)
    }

    pub fn gc_images(&mut self) -> Result<usize, Error> {
        eprintln!("Placeholder for gc_images");
        Ok(0)
    }

    pub fn gc_audit(&mut self) -> Result<(), Error> {
        eprintln!("Placeholder for gc_audit");
        Ok(())
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.root_dir
    }

    pub fn get_meta_by_key(&self, key: String) -> Result<Option<String>, Error> {
        eprintln!("Placeholder for get_meta_by_key: key={}", key);
        Ok(None)
    }

    // pub fn set_config_by_key(
    //     &mut self,
    //     key: String,
    //     value: String,
    // ) -> Result<Option<String>, Error> {
    //     eprintln!(
    //         "Placeholder for set_config_by_key: key={}, value={}",
    //         key, value
    //     );
    //     Ok(None)
    // }

    pub fn remove_meta_by_key(&mut self, key: String) -> Result<Option<String>, Error> {
        eprintln!("Placeholder for remove_meta_by_key: key={}", key);
        Ok(None)
    }

    pub fn remove_all_meta(&mut self) -> Result<(), Error> {
        eprintln!("Placeholder for remove_all_meta");
        Ok(())
    }

    pub fn status(&self) -> Result<String, Error> {
        eprintln!("Placeholder for status");
        Ok("Placeholder status".to_string())
    }

    pub fn remove_files(
        &mut self,
        query: &[String],
        dry_run: bool,
        recursive: bool,
        auto: bool,
        staged: bool,
        processed: bool,
    ) -> Result<RemoveResult, Error> {
        eprintln!("Placeholder for remove_files: query={:?}, dry_run={}, recursive={}, auto={}, staged={}, processed={}", query, dry_run, recursive, auto, staged, processed);
        Ok(RemoveResult {
            removed_files: 0,
            removed_chunks: 0,
        })
    }

    pub fn list_files<F, M, S>(
        &self,
        _filter: &F,
        _map: &M,
        _sort: &S,
    ) -> Result<Vec<FileSchema>, Error>
    where
        F: Fn(&FileSchema) -> bool,
        M: Fn(&FileSchema) -> FileSchema,
        S: Fn(&FileSchema) -> String,
    {
        eprintln!("Placeholder for list_files");
        Ok(vec![])
    }

    pub fn list_chunks<F, M, S>(
        &self,
        _filter: &F,
        _map: &M,
        _sort: &S,
    ) -> Result<Vec<ChunkSchema>, Error>
    where
        F: Fn(&ChunkSchema) -> bool,
        M: Fn(&ChunkSchema) -> ChunkSchema,
        S: Fn(&ChunkSchema) -> String,
    {
        eprintln!("Placeholder for list_chunks");
        Ok(vec![])
    }

    pub fn list_images<F, M, S>(
        &self,
        _filter: &F,
        _map: &M,
        _sort: &S,
    ) -> Result<Vec<ImageSchema>, Error>
    where
        F: Fn(&ImageSchema) -> bool,
        M: Fn(&ImageSchema) -> ImageSchema,
        S: Fn(&ImageSchema) -> usize,
    {
        eprintln!("Placeholder for list_images");
        Ok(vec![])
    }

    pub fn get_image_schema(
        &self,
        image_uid: Uid,
        with_bytes: bool,
    ) -> Result<ImageSchema, Error> {
        eprintln!(
            "Placeholder for get_image_schema: image_uid={}, with_bytes={}",
            image_uid, with_bytes
        );
        Err(Error::Internal(
            "Placeholder for get_image_schema".to_string(),
        ))
    }

    pub fn get_file_schema(
        &self,
        path: Option<PathBuf>,
        uid: Option<Uid>,
    ) -> Result<FileSchema, Error> {
        eprintln!(
            "Placeholder for get_file_schema: path={:?}, uid={:?}",
            path, uid
        );
        Err(Error::Internal(
            "Placeholder for get_file_schema".to_string(),
        ))
    }

//    pub fn get_chunks_of_file(&self, file_uid: Uid) -> Result<Vec<Uid>, Error> {
//        eprintln!("Placeholder for get_chunks_of_file: file_uid={}", file_uid);
//        Ok(vec![])
//    }

    // pub fn get_images_of_file(&self, file_uid: Uid) -> Result<Vec<Uid>, Error> {
    //     eprintln!("Placeholder for get_images_of_file: file_uid={}", file_uid);
    //     Ok(vec![])
    // }

    pub fn get_tfidf_by_file_uid(
        &self,
        file_uid: Uid,
    ) -> Result<ProcessedDoc, Error> {
        eprintln!(
            "Placeholder for get_tfidf_by_file_uid: file_uid={}",
            file_uid
        );
        Err(Error::Internal(
            "Placeholder for get_tfidf_by_file_uid".to_string(),
        ))
    }

    // pub fn get_tfidf_by_chunk_uid(
    //     &self,
    //     chunk_uid: Uid,
    // ) -> Result<ProcessedDoc, Error> {
    //     eprintln!(
    //         "Placeholder for get_tfidf_by_chunk_uid: chunk_uid={}",
    //         chunk_uid
    //     );
    //     Err(Error::Internal(
    //         "Placeholder for get_tfidf_by_chunk_uid".to_string(),
    //     ))
    // }

//    pub fn get_chunk_by_uid(&self, chunk_uid: Uid) -> Result<crate::chunk::Chunk, Error> {
//        eprintln!("Placeholder for get_chunk_by_uid: chunk_uid={}", chunk_uid);
//        Err(Error::Internal(
//            "Placeholder for get_chunk_by_uid".to_string(),
//        ))
//    }

    // pub fn get_merged_chunk_of_file(
    //     &self,
    //     file_uid: Uid,
    // ) -> Result<crate::chunk::RenderedChunk, Error> {
    //     eprintln!(
    //         "Placeholder for get_merged_chunk_of_file: file_uid={}",
    //         file_uid
    //     );
    //     Err(Error::Internal(
    //         "Placeholder for get_merged_chunk_of_file".to_string(),
    //     ))
    // }

    

    pub async fn add_files_command(
        &mut self,
        files: &[String],
        add_mode: Option<AddMode>,
        dry_run: bool,
    ) -> Result<AddResult, Error> {
        eprintln!(
            "Placeholder for add_files_command: files={:?}, add_mode={:?}, dry_run={}",
            files, add_mode, dry_run
        );
        Ok(AddResult {
            added_files: 0,
            added_chunks: 0,
        })
    }

    // pub fn get_initial_models() -> Result<Vec<ragit_api::ModelRaw>, Error> {
    //     eprintln!("Placeholder for get_initial_models");
    //     Ok(vec![])
    // }

    // pub fn load_config_from_home<T: serde::de::DeserializeOwned>(
    //     file_name: &str,
    // ) -> Result<Option<T>, Error> {
    //     eprintln!(
    //         "Placeholder for load_config_from_home: file_name={}",
    //         file_name
    //     );
    //     Ok(None)
    // }

    // pub fn get_all_configs(&self) -> Result<Vec<(String, String)>, Error> {
    //     eprintln!("Placeholder for get_all_configs");
    //     Ok(vec![])
    // }

    // pub fn set_config_by_key(
    //     &mut self,
    //     key: String,
    //     value: String,
    // ) -> Result<Option<String>, Error> {
    //     eprintln!(
    //         "Placeholder for set_config_by_key: key={}, value={}",
    //         key, value
    //     );
    //     Ok(None)
    // }

    pub async fn pull(
        &mut self,
        include_configs: bool,
        include_prompts: bool,
        quiet: bool,
    ) -> Result<PullResult, Error> {
        eprintln!(
            "Placeholder for pull: include_configs={}, include_prompts={}, quiet={}",
            include_configs, include_prompts, quiet
        );
        Ok(PullResult::AlreadyUpToDate)
    }

    pub async fn push(
        &mut self,
        remote: String,
        quiet: bool,
    ) -> Result<PushResult, Error> {
        eprintln!("Placeholder for push: remote={}, quiet={}", remote, quiet);
        Ok(PushResult::AlreadyUpToDate)
    }

    pub async fn clone(&mut self, url: &str, depth: Option<usize>) -> Result<(), Error> {
        eprintln!("Placeholder for clone: url={}, depth={:?}", url, depth);
        Ok(())
    }

    // pub fn find_lowest_cost_model(&self) -> Option<&ragit_api::Model> {
    //     eprintln!("Placeholder for find_lowest_cost_model");
    //     None
    // }
}