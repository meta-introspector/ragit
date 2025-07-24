pub mod commands;
pub mod file;
pub mod tfidf;
pub mod answer_query_with_chunks;
pub mod get_prompt;
pub mod index_load;
pub mod index_load_chunks_or_tfidf;
pub mod index_load_or_init;
pub mod index_new;
pub mod index_pull;
pub mod index_push;
pub mod index_remove_file_index;
pub mod index_run_tfidf;
pub mod index_save_to_file;
pub mod index_uid;
pub mod load_mode;
pub mod model_management;
pub mod muse_logic;
pub mod prompt_management;
pub mod index_file_schema;
pub mod index_image_schema;

pub mod raw_request;
pub mod rephrase_multi_turn;
pub mod retrieve_chunks;
pub mod summaries_to_chunks;

pub use ragit_readers::{FileReader, ImageDescription};
pub use tfidf::{consume_processed_doc, ProcessedDoc, TfidfResult, TfidfState};
pub use crate::prelude::*;
pub use ragit_config::BuildConfig;
pub use sha3::{Digest, Sha3_256};

pub type Term = String;
pub type Weight = f32;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum IIStatus {
    None,
    Outdated,
    Complete,
    Ongoing(Uid),
}

#[derive(Default)]
struct IIBuildState {
    total_uid: usize,
    buffer_uid: usize,
    buffer_term: usize,
    buffer_flush: usize,
}

fn hash(term: &Term) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(term.as_bytes());
    format!("{:064x}", hasher.finalize())
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Index {
    pub ragit_version: String,
    pub chunk_count: usize,
    pub staged_files: Vec<PathBuf>,
    pub processed_files: HashMap<PathBuf, Uid>,
    pub curr_processing_file: Option<PathBuf>,
    pub repo_url: Option<String>,
    pub ii_status: IIStatus,
    pub uid: Option<Uid>,
    pub summary: Option<ragit_commands::summary::Summary>,

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