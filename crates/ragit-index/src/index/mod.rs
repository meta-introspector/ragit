pub use ::serde::*;
pub mod commands;
pub mod get_prompt;
pub mod index_chunk_methods;
// pub mod index_add_file_to_index;
// pub mod index_add_staged_file;
// pub mod index_chunk_access;
pub mod query_logic;
pub mod index_file_schema;
pub mod index_image_schema;
pub mod index_load;
pub mod index_load_chunks_or_tfidf;
pub mod index_load_or_init;
pub mod index_new;
pub mod index_remove_file_index;
pub mod index_run_tfidf;
pub mod index_save_to_file;
pub mod index_uid;
pub mod model_management;
pub mod muse_logic;
pub mod prompt_management;
pub mod raw_request;
pub mod rephrase_multi_turn;
pub mod retrieve_chunks;
pub mod summaries_to_chunks;

pub use ragit_readers::*;
pub mod tfidf;
pub use tfidf::*;
pub use crate::prelude::*;
pub use ragit_config::BuildConfig;

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
