use anyhow::Result;
use std::path::PathBuf;
use sysinfo::System;
use ragit_index_types::index_struct::Index;
use ragit_utils::memory_utils::{print_memory_usage, check_memory_limit};
use crate::bootstrap_commands::build_index_logic::get_staged_files::get_staged_files;
use text_splitter::TextSplitter;
use std::fs;
use ragit_types::build_config::BuildConfig;
use ragit_types::fixed_types::fixed_chunk_struct::FixedChunk;
use ragit_types::uid::Uid;
use ragit_types::chunk::chunk_source::ChunkSource;

pub async fn build_index(
    verbose: bool,
    _temp_dir: &PathBuf,
    actual_root_dir: &PathBuf,
    index: &mut Index,
    _max_iterations: Option<usize>,
    sys: &mut System,
    max_memory_gb: Option<u64>,
    last_snapshot_data: &mut Option<(u64, u64, u64)>,
) -> Result<(), anyhow::Error> {
    if verbose {
        println!("bootstrap_index_self: Running rag build");
        println!("bootstrap_index_self: Before ragit_index_effects::build (placeholder)");
        print_memory_usage(sys, "Before ragit_index_effects::build", last_snapshot_data);
    }
    check_memory_limit(sys, max_memory_gb, "Before ragit_index_effects::build")?;

    let staged_files = get_staged_files(index)?;
    let build_config = BuildConfig::default();
    let splitter = TextSplitter::default();

    if verbose {
        println!("bootstrap_index_self: Iterating through staged files for chunking and indexing.");
    }
    let staged_files_cloned = index.staged_files.clone();
    for file_path_buf in &staged_files_cloned {
        let content = fs::read_to_string(file_path_buf)?;
        let chunks = splitter.chunks(&content, build_config.chunk_size);
        let file_path_str = file_path_buf.to_string_lossy().to_string();
        let mut chunk_index = 0;
        for chunk_data in chunks {
            let new_chunk = FixedChunk {
                data: chunk_data.into(),
                file: file_path_str.clone().into(),
                index: chunk_index,
                uid: Uid::new_from_slice(chunk_data.as_bytes()), // Generate UID from chunk data
                char_len: chunk_data.len(),
                source: ChunkSource::File { path: file_path_str.clone(), index: chunk_index, page: None },
                ..FixedChunk::dummy()
            };
            index.add_chunk(new_chunk);
            chunk_index += 1;
        }
    }

    if verbose {
        println!("bootstrap_index_self: After ragit_index_effects::build (placeholder)");
        println!("bootstrap_index_self: Built index (placeholder)");
        print_memory_usage(sys, "After ragit_index_effects::build", last_snapshot_data);
    }
    check_memory_limit(sys, max_memory_gb, "After ragit_index_effects::build")?;
    Ok(())
}