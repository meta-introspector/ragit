pub use anyhow::Result;
pub use chrono::{Days, Local as ChronoLocal};
pub use serde::{Deserialize, Serialize};
pub use serde_json::{Map, Value as JsonValue};
pub use std::collections::{HashMap, HashSet};
pub use std::fmt;
pub use std::path::{Path, PathBuf};
pub use std::str::FromStr;
pub use tokio::sync::mpsc;

// Re-exports from ragit-api
pub use ragit_api::{get_model_by_name, list_models, Model, ModelRaw, Request};

// Re-exports from ragit-types
pub use ragit_types::ApiError;
pub use ragit_types::{Chunk, ChunkBuildInfo, FileSchema, ImageSchema, Uid, AuditRecordAt, QueryResponse, QueryTurn, AddMode};
pub use ragit_types::chunk::chunk_source::ChunkSource;
pub use ragit_types::chunk::rendered_chunk::{MultiModalContent, RenderedChunk};
pub use ragit_types::api_config::ApiConfig;
pub use ragit_types::query_config::QueryConfig; // Corrected import for QueryConfig
pub use ragit_types::model::Model as TypesModel;
pub use ragit_types::ii_status::IIStatus;
pub use ragit_types::summary::{Summary, SummaryMode};
pub use ragit_types::processed_doc::ProcessedDoc;
pub use ragit_types::json_type::JsonType; // Added JsonType

// Re-exports from ragit-fs
pub use ragit_fs::{create_dir_all, exists, extension, file_name, file_size, get_relative_path, into_abs_path, is_dir, join as fs_join, join3, join4, normalize, read_bytes, read_bytes_offset, read_dir, remove_dir_all, remove_file, set_extension, try_create_dir, write_bytes, write_string as fs_write_string, WriteMode as FsWriteMode, read_string as fs_read_string};

// Re-exports from ragit-pdl
pub use ragit_pdl::escape_pdl_tokens::escape_pdl_tokens;
pub use ragit_pdl::parse_pdl::parse_pdl_logic as pdl_parse_pdl;
pub use ragit_pdl::parse_pdl_from_file::parse_pdl_from_file;
pub use ragit_pdl::pdl_struct::Pdl as PdlStruct;
pub use ragit_pdl::pdl_types::{Message, Role}; // Re-export Message and Role from pdl_types

// Re-exports from ragit-utils
pub use ragit_utils::cli_types::{ArgParser, ArgType, ArgCount, Span, ParsedArgs};
pub use ragit_utils::error::{CliError, Error, ErrorKind};
pub use ragit_utils::doc_utils::get_doc_content;
pub use ragit_utils::project_root::find_root;
pub use ragit_utils::version_info::VersionInfo;
pub use ragit_utils::path_utils::str_to_pathbuf; // Re-export str_to_pathbuf

// Re-exports from ragit-index-types
pub use ragit_index_types::index_struct::Index;
pub use ragit_index_types::load_mode::LoadMode;
pub use ragit_index_types::MergeMode;

// Re-exports from ragit-query
pub use ragit_query::query_helpers::uid_query;
pub use ragit_query::UidQueryConfig; // Re-export UidQueryConfig

// Re-exports from ragit-api (TestModel)
pub use ragit_api::TestModel;

// Re-exports for schema parsing and rendering
pub use ragit_pdl::schema::parse::parse_schema;
pub use ragit_pdl::schema::render::render_pdl_schema;

// Re-exports for QA System
pub use ragit_qa_system::ModelQASystem;