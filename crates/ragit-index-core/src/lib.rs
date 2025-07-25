use std::path::PathBuf;
use ragit_types::api_config::ApiConfig;
use ragit_error::ApiError;
use ragit_types::query::QueryConfig;
use ragit_types::uid::Uid;
use ragit_types::ii_status::IIStatus;
use ragit_types::summary::{Summary, SummaryMode};
use ragit_config::BuildConfig;
use ragit_utils::version_info::VersionInfo;

use ragit_api::Model;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ragit_types::chunk::chunk_struct::Chunk;
use ragit_tfidf::TfidfResult;
use tokio::task::JoinSet;
use ragit_index_io::get_chunk_by_uid::get_chunk_by_uid;
use ragit_api::AuditRecord;
use chrono::{DateTime, Utc};
use serde_json::Value;
use ragit_utils::constant::INDEX_FILE_NAME;

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
        self.summary.as_ref().map(|s| s.content.as_str())
    }

    pub fn get_ragit_version_info(&self) -> VersionInfo {
        // Placeholder implementation
        VersionInfo {
            version: self.ragit_version.clone(),
            compatible: true, // Assume compatible for now
        }
    }

    pub fn migrate(&mut self, to_version: String) -> Result<(), ApiError> {
        // Placeholder implementation
        eprintln!("Migrating to version: {}", to_version);
        // In a real scenario, this would contain logic to migrate the index data
        // based on the `to_version`.
        Ok(())
    }

    pub async fn summary(&mut self, mode: Option<SummaryMode>) -> Result<Option<String>, ApiError> {
        // Placeholder implementation for summary generation
        eprintln!("Generating summary with mode: {:?}", mode);
        // In a real scenario, this would involve LLM calls to generate the summary.
        // For now, let's return a dummy summary.
        self.summary = Some(Summary { content: String::from("This is a generated summary.") });
        Ok(self.summary.clone().map(|s| s.content))
    }

    pub fn get_all_meta(&self) -> Result<HashMap<String, String>, ApiError> {
        let result = HashMap::new();

        Ok(result)
    }

    pub fn dummy() -> Self {
        Index {
            ragit_version: String::new(),
            chunk_count: 0,
            staged_files: vec![],
            processed_files: HashMap::new(),
            curr_processing_file: None,
            repo_url: None,
            ii_status: IIStatus::default(),
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

    pub async fn retrieve_chunks(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<Chunk>, ApiError> {
        // Assuming run_tfidf will be moved to ragit-tfidf or ragit-index-tfidf
        // For now, we'll use a placeholder.
        // TODO: Call run_tfidf from ragit-tfidf or ragit-index-tfidf
        let tfidf_results: Vec<TfidfResult> = Vec::new(); // Placeholder
        let mut chunks = Vec::with_capacity(tfidf_results.len());
        let mut join_set = JoinSet::new();

        for tfidf_result in tfidf_results {
            let index_clone = self.clone();
            join_set.spawn(async move {
                get_chunk_by_uid(&index_clone, tfidf_result.doc_id)
            });
        }

        while let Some(res) = join_set.join_next().await {
            chunks.push(res.unwrap()?);
        }

        Ok(chunks)
    }

    pub fn audit(
        &self,
        since: Option<DateTime<chrono::Local>>,
    ) -> Result<HashMap<String, AuditRecord>, ApiError> {
        let mut result = HashMap::new();
        let audit_path =
            ragit_utils::ragit_path_utils::get_rag_path(&self.root_dir, &PathBuf::from("audit"))?.join("audit");
        if !audit_path.exists() {
            return Ok(result);
        }
        for entry in std::fs::read_dir(audit_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                continue;
            }
            let file_name = path.file_name().unwrap().to_string_lossy();
            let parts: Vec<&str> = file_name.splitn(2, '_').collect();
            if parts.len() != 2 {
                continue;
            }
            let timestamp = parts[0].parse::<i64>()?;
            let category = parts[1].to_string();
            if let Some(since) = since {
                if DateTime::<Utc>::from_timestamp(timestamp, 0).unwrap() < since.to_utc() {
                    continue;
                }
            }
            let audit: AuditRecord = serde_json::from_str(&std::fs::read_to_string(&path)?)?;
            *result.entry(category).or_default() += audit;
        }
        Ok(result)
    }

    pub fn get_config_by_key(&self, key: String) -> Result<Value, ApiError> {
        let value = match key.as_str() {
            "model" => serde_json::to_value(&self.api_config.model)?,
            "timeout" => serde_json::to_value(&self.api_config.timeout)?,
            "sleep_between_retries" => {
                serde_json::to_value(&self.api_config.sleep_between_retries)?
            }
            "max_retry" => serde_json::to_value(&self.api_config.max_retry)?,
            "sleep_after_llm_call" => serde_json::to_value(&self.api_config.sleep_after_llm_call)?,
            "dump_log" => serde_json::to_value(&self.api_config.dump_log)?,
            "dump_api_usage" => serde_json::to_value(&self.api_config.dump_api_usage)?,
            "enable_muse_mode" => serde_json::to_value(&self.api_config.enable_muse_mode)?,
            "throttling_safety_margin" => {
                serde_json::to_value(&self.api_config.throttling_safety_margin)?
            }
            "max_chunk_size" => serde_json::to_value(&self.build_config.chunk_size)?,
            "max_summary_len" => serde_json::to_value(&self.build_config.max_summary_len)?,
            "min_summary_len" => serde_json::to_value(&self.build_config.min_summary_len)?,
            "enable_ii" => serde_json::to_value(&self.query_config.enable_ii)?,
            _ => return Err(ApiError::InvalidConfigKey(key.clone())),
        };
        Ok(value)
    }

    pub fn set_config_by_key(
        &mut self,
        key: String,
        value: String,
    ) -> Result<Option<Value>, ApiError> {
        let previous_value = match key.as_str() {
            "model" => {
                let prev = self.api_config.model.clone();
                self.api_config.model = value;
                serde_json::to_value(prev).ok()
            }
            "timeout" => {
                let prev = self.api_config.timeout;
                self.api_config.timeout = Some(value.parse()?);
                serde_json::to_value(prev).ok()
            }
            "sleep_between_retries" => {
                let prev = self.api_config.sleep_between_retries;
                self.api_config.sleep_between_retries = value.parse()?;
                serde_json::to_value(prev).ok()
            }
            "max_retry" => {
                let prev = self.api_config.max_retry;
                self.api_config.max_retry = value.parse()?;
                serde_json::to_value(prev).ok()
            }
            "sleep_after_llm_call" => {
                let prev = self.api_config.sleep_after_llm_call;
                self.api_config.sleep_after_llm_call = Some(value.parse()?);
                serde_json::to_value(prev).ok()
            }
            "dump_log" => {
                let prev = self.api_config.dump_log;
                self.api_config.dump_log = value.parse()?;
                serde_json::to_value(prev).ok()
            }
            "dump_api_usage" => {
                let prev = self.api_config.dump_api_usage;
                self.api_config.dump_api_usage = value.parse()?;
                serde_json::to_value(prev).ok()
            }
            "enable_muse_mode" => {
                let prev = self.api_config.enable_muse_mode;
                self.api_config.enable_muse_mode = value.parse()?;
                serde_json::to_value(prev).ok()
            }
            "throttling_safety_margin" => {
                let prev = self.api_config.throttling_safety_margin;
                self.api_config.throttling_safety_margin = value.parse()?;
                serde_json::to_value(prev).ok()
            }
            "max_chunk_size" => {
                let prev = self.build_config.chunk_size;
                self.build_config.chunk_size = value.parse()?;
                serde_json::to_value(prev).ok()
            }
            "max_summary_len" => {
                let prev = self.build_config.max_summary_len;
                self.build_config.max_summary_len = value.parse()?;
                serde_json::to_value(prev).ok()
            }
            "min_summary_len" => {
                let prev = self.build_config.min_summary_len;
                self.build_config.min_summary_len = value.parse()?;
                serde_json::to_value(prev).ok()
            }
            "enable_ii" => {
                let prev = self.query_config.enable_ii;
                self.query_config.enable_ii = value.parse()?;
                serde_json::to_value(prev).ok()
            }
            _ => return Err(ApiError::InvalidConfigKey(key.clone())),
        };
        // self.save_to_file(self.root_dir.join(INDEX_FILE_NAME))?;
        Ok(previous_value)
    }

    pub fn get_all_configs(&self) -> Result<Vec<(String, Value)>, ApiError> {
        let mut configs = vec![];

        // ApiConfig
        configs.push((
            "model".to_string(),
            serde_json::to_value(&self.api_config.model)?,
        ));
        configs.push((
            "timeout".to_string(),
            serde_json::to_value(&self.api_config.timeout)?,
        ));
        configs.push((
            "sleep_between_retries".to_string(),
            serde_json::to_value(&self.api_config.sleep_between_retries)?,
        ));
        configs.push((
            "max_retry".to_string(),
            serde_json::to_value(&self.api_config.max_retry)?,
        ));
        configs.push((
            "sleep_after_llm_call".to_string(),
            serde_json::to_value(&self.api_config.sleep_after_llm_call)?,
        ));
        configs.push((
            "dump_log".to_string(),
            serde_json::to_value(&self.api_config.dump_log)?,
        ));
        configs.push((
            "dump_api_usage".to_string(),
            serde_json::to_value(&self.api_config.dump_api_usage)?,
        ));
        configs.push((
            "enable_muse_mode".to_string(),
            serde_json::to_value(&self.api_config.enable_muse_mode)?,
        ));
        configs.push((
            "throttling_safety_margin".to_string(),
            serde_json::to_value(&self.api_config.throttling_safety_margin)?,
        ));

        // BuildConfig
        configs.push((
            "max_chunk_size".to_string(),
            serde_json::to_value(&self.build_config.chunk_size)?,
        ));
        configs.push((
            "max_summary_len".to_string(),
            serde_json::to_value(&self.build_config.max_summary_len)?,
        ));
        configs.push((
            "min_summary_len".to_string(),
            serde_json::to_value(&self.build_config.min_summary_len)?,
        ));

        // QueryConfig
        configs.push((
            "enable_ii".to_string(),
            serde_json::to_value(&self.query_config.enable_ii)?,
        ));

        Ok(configs)
    }
}

/// 1. If you want to do something with chunks, use `LoadMode::QuickCheck`.
/// 2. If you have nothing to do with chunks, use `LoadMode::OnlyJson`.
/// 3. If something's broken and you don't want it to crash, use `LoadMode::Minimum`. It can still crash, though.
/// 4. If you want to be very sure that nothing's broken and you don't care about init-time, use `LoadMode::Check`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LoadMode {
    /// It only loads `index.json`. It doesn't care whether config files prompt files, or chunk files are broken.
    Minimum,

    /// It loads json files, but doesn't care whether the chunk files are broken.
    OnlyJson,

    /// It checks and auto-recovers if `self.curr_processing_file` is not None. If the value is not None,
    /// a previous build was interrupted and something could be broken.
    QuickCheck,

    /// It always checks and auto-recovers. You should be very careful, `check` and `auto-recover` are very expensive.
    Check,
}