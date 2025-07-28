use anyhow::Result;
use std::path::PathBuf;
use sysinfo::System;
use ragit_index_types::index_struct::Index;
use crate::bootstrap_commands::constants::CHUNK_PROCESSING_LIMIT;
// use crate::bootstrap_commands::memory_utils::{print_memory_usage, check_memory_limit};
use crate::bootstrap_commands::write_chunks_to_markdown::initialize_markdown_output;
use crate::bootstrap_commands::write_chunks_to_markdown::process_single_chunk;
use crate::bootstrap_commands::write_chunks_to_markdown::finalize_markdown_output;

pub async fn write_chunks_to_markdown(
    verbose: bool,
    temp_dir: &PathBuf,
    index: &Index,
    sys: &mut System,
    max_memory_gb: Option<u64>,
    last_process_memory_kb: &mut Option<u64>,
    max_iterations: Option<usize>,
) -> Result<(), anyhow::Error> {
    let mut initialize_call_count = 0;
    let mut process_call_count = 0;
    let mut finalize_call_count = 0;

    let (mut markdown_output, mut processed_chunks_count, all_chunk_files, total_chunk_files) = {
        initialize_call_count += 1;
        initialize_markdown_output::initialize_markdown_output(
            verbose,
            index,
            sys,
            max_memory_gb,
            last_process_memory_kb,
            initialize_call_count,
        ).await?
    };

    for chunk_path in all_chunk_files {
        if let Some(max_iter) = max_iterations {
            if processed_chunks_count >= max_iter {
                println!("bootstrap_index_self: Stopping chunk processing after {} chunks due to max_iterations limit.", max_iter);
                break;
            }
        }

        process_call_count += 1;
        process_single_chunk::process_single_chunk(
            verbose,
            &chunk_path,
            &mut markdown_output,
            processed_chunks_count,
            total_chunk_files,
            sys,
            max_memory_gb,
            last_process_memory_kb,
            process_call_count,
        ).await?;

        processed_chunks_count += 1;
    }

    finalize_call_count += 1;
    finalize_markdown_output::finalize_markdown_output(
        verbose,
        temp_dir,
        &markdown_output,
        sys,
        max_memory_gb,
        last_process_memory_kb,
        finalize_call_count,
    ).await?;

    Ok(())
}