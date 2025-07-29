use anyhow::Result;
use std::path::PathBuf;

use ragit_fs::{write_string, WriteMode};

use ragit_memory_monitor::MemoryMonitor;
use crate::bootstrap_commands::constants::CHUNKS_OUTPUT_FILE_NAME;

pub async fn finalize_markdown_output(
    verbose: bool,
    temp_dir: &PathBuf,
    markdown_output: &String,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
    call_count: usize,
) -> Result<(), anyhow::Error> {
    if verbose {
        memory_monitor.capture_and_log_snapshot(&format!("After chunk processing loop (Call: {})", call_count));
    }
    memory_monitor.check_memory_limit(max_memory_gb, &format!("After chunk processing loop (Call: {})", call_count))?;

    let chunks_file_path = temp_dir.join(CHUNKS_OUTPUT_FILE_NAME);
    write_string(chunks_file_path.to_str().unwrap(), markdown_output, WriteMode::CreateOrTruncate)?;
    if verbose {
        println!("bootstrap_index_self: Chunks written to {:?} (Call: {})", chunks_file_path, call_count);
    }
    Ok(())
}
