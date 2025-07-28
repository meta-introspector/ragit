use anyhow::Result;
use std::path::PathBuf;
use sysinfo::System;
use ragit_index_types::index_struct::Index;
use crate::bootstrap_commands::memory_utils::{print_memory_usage, check_memory_limit};
use ragit_index_storage;

pub async fn initialize_markdown_output(
    verbose: bool,
    index: &Index,
    sys: &mut System,
    max_memory_gb: Option<u64>,
    last_process_memory_kb: &mut Option<u64>,
    call_count: usize,
) -> Result<(String, usize, Vec<PathBuf>, usize), anyhow::Error> {
    if verbose {
        println!("bootstrap_index_self: Writing chunks to markdown file (Call: {})", call_count);
        print_memory_usage(sys, &format!("Before iterating chunk files (Call: {})", call_count), last_process_memory_kb);
    }
    check_memory_limit(sys, max_memory_gb, &format!("Before iterating chunk files (Call: {})", call_count))?;

    let markdown_output = String::new();
    let processed_chunks_count = 0;

    let all_chunk_files = ragit_index_storage::get_all_chunk_files(&index.root_dir)?;
    let total_chunk_files = all_chunk_files.len();

    Ok((markdown_output, processed_chunks_count, all_chunk_files, total_chunk_files))
}
