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
pub use ragit_types::chunk::chunk_schema::ChunkSchema; // Added for ChunkSchema fields
pub use ragit_types::chunk::chunk_source::ChunkSource;
pub use ragit_types::chunk::rendered_chunk::{MultiModalContent, RenderedChunk};
pub use ragit_types::api_config::ApiConfig;
pub use ragit_types::query_config::QueryConfig; // Corrected import for QueryConfig
pub use ragit_types::model::Model as TypesModel;
pub use ragit_types::ii_status::IIStatus;
pub use ragit_types::summary::{Summary, SummaryMode};
pub use ragit_types::processed_doc::ProcessedDoc;
pub use ragit_types::json_type::JsonType; // Added JsonType
pub use ragit_utils::query::Keywords; // Added Keywords

// Re-exports from ragit-fs
pub use ragit_fs::{create_dir_all, exists, extension, file_name, file_size, get_relative_path, into_abs_path, is_dir, join as fs_join, join3, join4, normalize, read_bytes, read_bytes_offset, read_dir, remove_dir_all, remove_file, set_extension, try_create_dir, write_bytes, write_string as fs_write_string, WriteMode as FsWriteMode, read_string as fs_read_string};

// Re-exports from ragit-pdl
pub use ragit_pdl::escape_pdl_tokens::escape_pdl_tokens;
pub use ragit_pdl::parse_pdl::parse_pdl_logic as pdl_parse_pdl;
pub use ragit_pdl::parse_pdl_from_file::parse_pdl_from_file;
pub use ragit_pdl::pdl_struct::Pdl as PdlStruct;
pub use ragit_types::pdl_types::{Message, Role};

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





// pub use ragit_index_types::index_impl::audit::audit as index_audit;
// pub use ragit_index_types::index_impl::build_ii::build_ii as index_build_ii;
// pub use ragit_index_types::index_impl::config_methods::get_all_configs as index_get_all_configs;
// 
// pub use ragit_index_types::index_impl::get_images_of_file::get_images as index_get_images;
// pub use ragit_index_types::index_impl::get_summary::get_summary as index_get_summary;
// pub use ragit_index_types::index_impl::list_chunks::list_chunks as index_list_chunks;
// pub use ragit_index_types::index_impl::list_files::list_files as index_list_files;
// pub use ragit_index_types::index_impl::list_images::list_images as index_list_images;
// pub use ragit_index_types::index_impl::list_models::list_models as index_list_models;
// pub use ragit_index_types::index_impl::list_terms::list_terms as index_list_terms;
// pub use ragit_index_types::index_impl::merge::merge as index_merge;
// pub use ragit_index_types::index_impl::model_access_methods::search_remote_models as index_search_remote_models;
// pub use ragit_index_types::index_impl::model_access_methods::fetch_remote_models as index_fetch_remote_models;
// pub use ragit_index_types::index_impl::model_access_methods::fetch_all_remote_models as index_fetch_all_remote_models;
// pub use ragit_index_types::index_impl::remove_local_model::remove_local_model as index_remove_local_model;
// pub use ragit_index_types::index_impl::remove_all_local_models::remove_all_local_models as index_remove_all_local_models;
// pub use ragit_index_types::index_impl::reset_ii::reset_ii as index_reset_ii;
// pub use ragit_index_types::index_impl::gc_logs::gc_logs as index_gc_logs;
// pub use ragit_index_types::index_impl::gc_images::gc_images as index_gc_images;
// pub use ragit_index_types::index_impl::gc_audit::gc_audit as index_gc_audit;
// pub use ragit_index_types::index_impl::init::init as index_init;
// pub use ragit_index_types::index_impl::pull::pull as index_pull;
// pub use ragit_index_types::index_impl::push::push as index_push;
pub use ragit_index_types::index_impl::set_config_by_key::index_set_config_by_key;
// pub use ragit_index_types::index_impl::add_file_index::index_add_file_index; // Added for add_files_command

// Re-exports from ragit-index-core
pub use ragit_index_core::add_files::add_files_command;
pub use ragit_index_core::clone::clone_command;



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
