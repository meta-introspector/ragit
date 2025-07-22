use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};


pub mod prelude;
pub mod agent;
pub mod api_config;
pub mod chunk;
pub mod constant;
pub mod error_reporting;
pub mod error;
pub mod imports;

pub mod log_qa_results;




pub mod prompts;
pub mod query;

pub mod tree;
pub mod uid;




pub use ragit_utils::agent::file_tree::FileTree;

pub use ragit_utils::chunk::{Chunk, ChunkBuildInfo, ChunkSource, MultiModalContent, merge_and_convert_chunks, RenderedChunk};
pub use ragit_utils::chunk::utils::into_multi_modal_contents;


pub use ragit_utils::index::{IIStatus, ProcessedDoc};
pub use ragit_utils::index::index_struct::Index;
pub use ragit_utils::index::load_mode::LoadMode;

pub use ragit_utils::api_config::ApiConfig;
pub use ragit_utils::query::{Keywords, MultiTurnSchema, QueryConfig, QueryResponse, QueryTurn};
pub use ragit_utils::uid::Uid;
pub use ragit_utils::uid::{UidQueryConfig, UidQueryResult};


// My rules for version numbers
// Let's say I'm working on 0.1.2
//
// |                             | Cargo.toml  | this constant  |
// |-----------------------------|-------------|----------------|
// | working on 0.1.2            | 0.1.2       | "0.1.2-dev"    |
// | published version of 0.1.2  | 0.1.2       | "0.1.2"        |
// | after publishing 0.1.2      | 0.1.3       | "0.1.3-dev"    |
//
// Feel free to use whatever rules for your branches. But please keep version numbers
// distinguishable, so that chunks generated from your branches can easily be identified.

