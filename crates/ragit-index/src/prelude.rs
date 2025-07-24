// Import other preludes
pub use ragit_types::prelude::*;
pub use ragit_api::prelude::{self as ragit_api_prelude, Error as ApiError};

// Index-specific items
pub use crate::index::{Index, IIStatus, BuildConfig};

// Reader-specific items
pub use ragit_readers::ImageDescription;

// Query response items
pub use ragit_model_query_response::{ModelQueryResponse as QueryResponse, QueryTurn};

// Filesystem items
pub use ragit_fs::{exists, create_dir, create_dir_all, join, join3, join4, read_bytes, read_dir, read_string, remove_file, write_bytes, write_string, WriteMode, file_name, parent, extension, get_relative_path, set_extension};

// Std library items
pub use std::collections::{HashMap, HashSet};
pub use std::ops::{Deref, DerefMut};

// Other crates
pub use lazy_static::lazy_static;
pub use regex::Regex;

pub use anyhow::{anyhow, Result};
pub use ragit_utils::{
    api_config::{ApiConfig, PartialApiConfig},
    constant::*,
    error::Error,
    prompts::PROMPTS,
    query::{Keywords, MultiTurnSchema, QueryConfig},
    path_utils::{get_ii_path, get_normalized_abs_pathbuf, get_rag_path, get_uid_path, join3_paths, join_paths},
    cli_types::{ArgParser, ArgType, ArgCount, ParsedArgs}
};
