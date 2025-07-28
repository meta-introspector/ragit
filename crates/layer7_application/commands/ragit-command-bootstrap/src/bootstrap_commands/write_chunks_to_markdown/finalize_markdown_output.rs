use anyhow::Result;
use std::path::PathBuf;
use sysinfo::System;
use ragit_fs::{write_string, WriteMode};
use crate::bootstrap_commands::memory_utils::{print_memory_usage, check_memory_limit};
use crate::bootstrap_commands::constants::CHUNKS_OUTPUT_FILE_NAME;

pub async fn finalize_markdown_output(
    verbose: bool,
    temp_dir: &PathBuf,
    markdown_output: &String,
    sys: &mut System,
    max_memory_gb: Option<u64>,
    last_process_memory_kb: &mut Option<u64>,
    call_count: usize,
) -> Result<(), anyhow::Error> {
    if verbose {
        print_memory_usage(sys, &format!("After chunk processing loop (Call: {})", call_count), last_process_memory_kb);
    }
    check_memory_limit(sys, max_memory_gb, &format!("After chunk processing loop (Call: {})", call_count))?;

    let chunks_file_path = temp_dir.join(CHUNKS_OUTPUT_FILE_NAME);
    write_string(chunks_file_path.to_str().unwrap(), markdown_output, WriteMode::CreateOrTruncate)?;
    if verbose {
        println!("bootstrap_index_self: Chunks written to {:?} (Call: {})", chunks_file_path, call_count);
    }
    Ok(())
}
