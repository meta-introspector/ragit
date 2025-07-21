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
pub mod main;

pub mod prompts;
pub mod query;

pub mod tree;
pub mod uid;



pub use ragit_utils::agent::action::{Action as AgentAction, ActionResult, ActionState, ActionTrace, ArgumentTurn, SearchType};
pub use ragit_utils::agent::file_tree::FileTree;
pub use crate::api_config::ApiConfig;
pub use ragit_utils::chunk::{Chunk, ChunkBuildInfo, ChunkSource, MultiModalContent, into_multi_modal_contents, merge_and_convert_chunks, RenderedChunk};
pub use constant::*;
pub use ragit_utils::error::Error;
pub use ragit_utils::index::{AddMode, AddResult, Audit, BuildConfig, BuildResult, IIStatus, ImageDescription, Index, LoadMode, MergeMode, MergeResult, ProcessedDoc, PullResult, PushResult, RecoverResult, RemoveResult, Summary, SummaryMode, TfidfResult, VersionInfo, get_compatibility_warning,};
pub use ragit_utils::query::{Keywords, MultiTurnSchema, QueryConfig, QueryResponse, QueryTurn,};
pub use ragit_utils::uid::{Uid, UidQueryConfig, UidQueryResult};
pub use ragit_schema::{FileSchema, ImageSchema, Prettify};

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
pub const VERSION: &str = "0.4.2-dev";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BuildOptions {
    pub version: String,
    pub profile: String,  // debug | release | production
    pub features: HashMap<String, bool>,
}

pub fn get_build_options() -> BuildOptions {
    let profile = if cfg!(feature = "production") {
        "production"
    } else if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };

    BuildOptions {
        version: VERSION.to_string(),
        profile: profile.to_string(),
        features: vec![
            (String::from("csv"), cfg!(feature = "csv")),
            (String::from("korean"), cfg!(feature = "korean")),
            (String::from("pdf"), cfg!(feature = "pdf")),
            (String::from("svg"), cfg!(feature = "svg")),
        ].into_iter().collect(),
    }
}
