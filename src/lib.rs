use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub mod constant;
pub mod error_reporting;
pub mod imports;
pub mod prelude;

pub mod prompts;
pub mod query;

pub mod tree;

pub use ragit_agent::file_tree::FileTree;

pub use ragit_index::chunk_methods::utils::into_multi_modal_contents;
pub use ragit_types::{Chunk, ChunkBuildInfo, chunk::{ChunkSource, MultiModalContent, RenderedChunk}};
pub use ragit_index::chunk_methods::utils::merge_and_convert_chunks;

pub use ragit_index_io::index_struct::Index;
pub use ragit_index::load_mode::LoadMode;


pub use ragit_types::api_config::ApiConfig;
pub use ragit_utils::query::{Keywords, MultiTurnSchema, QueryConfig};
pub use ragit_model_query_response::{ModelQueryResponse, QueryTurn};
pub use ragit_types::uid::Uid;
pub use ragit_index::query_helpers::{UidQueryConfig, UidQueryResult};

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