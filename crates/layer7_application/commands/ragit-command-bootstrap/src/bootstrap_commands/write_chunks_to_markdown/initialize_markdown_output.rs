use anyhow::Result;
use std::path::PathBuf;
use sysinfo::System;
use ragit_index_types::index_struct::Index;
use ragit_utils::memory_utils::{print_memory_usage, check_memory_limit};
use ragit_types::fixed_types::fixed_chunk_struct::FixedChunk;

pub async fn initialize_markdown_output(
    verbose: bool,
    index: &Index,
    sys: &mut System,
    max_memory_gb: Option<u64>,
    last_snapshot_data: &mut Option<(u64, u64, u64)>,
    call_count: usize,
) -> Result<(String, usize, Vec<FixedChunk>, usize), anyhow::Error> {
    if verbose {
        println!("bootstrap_index_self: Writing chunks to markdown file (Call: {})", call_count);
        ragit_utils::memory_utils::print_memory_usage(sys, &format!("Before iterating chunk files (Call: {})", call_count), last_snapshot_data);
    }
    check_memory_limit(sys, max_memory_gb, &format!("Before iterating chunk files (Call: {})", call_count))?;

    let markdown_output = String::new();
    let processed_chunks_count = 0;

    let all_chunks = index.chunks.clone();
    let total_chunks = all_chunks.len();

    Ok((markdown_output, processed_chunks_count, all_chunks, total_chunks))
}
