use anyhow::Result;
use std::path::PathBuf;
use ragit_types::{Uid};
use ragit_types::fixed_types::fixed_chunk_struct::{FixedChunk, FixedChunkBuildInfo};

use ragit_types::fixed_types::fixed_vec::FixedVec;
use ragit_types::chunk::ChunkSource;
use chrono;

pub fn create_chunk_from_file_data(
    file_content: String,
    file_path: String,
    chunk_index: usize,
    file_path_buf: &PathBuf,
) -> Result<FixedChunk, anyhow::Error> {
    let chunk = FixedChunk {
        data: file_content.into(),
        images: FixedVec::new(),
        char_len: 0,
        image_count: 0,
        title: format!("Chunk from {}", file_path).into(),
        summary: String::new().into(),
        file: file_path.clone().into(),
        index: chunk_index,
        source: ChunkSource::File { path: file_path_buf.to_string_lossy().to_string(), index: chunk_index, page: Some(0) },
        uid: Uid::new_from_slice(file_path.as_bytes()),
        build_info: FixedChunkBuildInfo::default(),
        timestamp: chrono::Utc::now().timestamp(),
        searchable: true,
    };
    Ok(chunk)
}