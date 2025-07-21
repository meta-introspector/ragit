use crate::error::Error;
use crate::prompts::Prompt;
use ragit_utils::query::QueryResponse;
use ragit_utils::Uid;

use ragit_utils::chunk::{Chunk, ChunkBuildInfo, ChunkSource, MultiModalContent, AtomicToken, RenderedChunk};
use ragit_utils::index::index_struct::Index;
use ragit_utils::path_utils;
use ragit_utils::index::tfidf;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use flate2::read::{GzDecoder, GzEncoder};
use flate2::Compression;
use std::io::Read;
use ragit_fs::{read_bytes, write_bytes, WriteMode, parent, exists, try_create_dir, set_extension};


