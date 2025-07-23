pub use crate::commands::ls_chunks::ls_chunks_command_main;
pub use crate::commands::ls_files::ls_files_command_main;
pub use crate::commands::ls_images::ls_images_command_main;
pub use crate::commands::ls_models::ls_models_command_main;
pub use crate::commands::ls_terms::ls_terms_command_main;
pub use ragit_uid::Uid;
pub use ragit_utils::api_config::ApiConfig;
pub use ragit_utils::chunk::utils::into_multi_modal_contents;
pub use ragit_utils::chunk::{
    merge_and_convert_chunks, Chunk, ChunkBuildInfo, ChunkSource, MultiModalContent, RenderedChunk,
};
pub use ragit_utils::cli_types::{ArgCount, ArgParser, ArgType, ParsedArgs, Span};
pub use ragit_utils::error::{CliError, Error, ErrorKind};
pub use ragit_utils::index::commands::{
    get_compatibility_warning, AddMode, BuildResult, MergeMode, MergeResult, PullResult,
    PushResult, RemoveResult, SummaryMode,
};
pub use ragit_utils::index::index_struct::Index;
pub use ragit_utils::index::load_mode::LoadMode;
pub use ragit_utils::query::{Keywords, MultiTurnSchema, QueryConfig, QueryResponse, QueryTurn};
pub use ragit_utils::string_utils::get_closest_string;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BuildOptions {
    pub version: String,
    pub profile: String, // debug | release | production
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
        ]
        .into_iter()
        .collect(),
    }
}
pub use anyhow::Error as AnyhowError;
pub use async_recursion::async_recursion;
pub use chrono::{Days, Local};
pub use csv::Error as CsvError;
pub use image::ImageError;
pub use lazy_static::lazy_static;
pub use ragit_api::Error as ApiError;
pub use ragit_api::{
    get_model_by_name, list_models, Model, ModelQAResult, ModelQASystem, ModelRaw, QualityScores,
    Request,
};
pub use ragit_fs::{
    exists, extension, file_name, file_size, get_relative_path, into_abs_path, is_dir, join, join3,
    join4, normalize, read_bytes, read_bytes_offset, read_dir, remove_dir_all, remove_file,
    set_extension, try_create_dir, write_bytes, write_string, WriteMode,
};
pub use ragit_pdl::{
    encode_base64, escape_pdl_tokens, parse_pdl, parse_pdl_from_file, parse_schema,
    render_pdl_schema, JsonType, Message, Pdl, Role,
};
pub use ragit_utils::doc_utils::get_doc_content;
pub use ragit_utils::index::commands::AddMode;
pub use ragit_utils::project_root::find_root;
pub use regex::Regex;
pub use reqwest::Error as ReqwestError;
pub use serde::{Deserialize, Serialize};
pub use serde_json::{Map, Value};
pub use sha3::{Digest, Sha3_256};
pub use std::collections::{HashMap, HashSet};
pub use std::fmt;
pub use std::path::{Path, PathBuf};
pub use std::str::FromStr;
pub use tera::Error as TeraError;
pub use tokio::sync::mpsc;
pub use url::ParseError;
