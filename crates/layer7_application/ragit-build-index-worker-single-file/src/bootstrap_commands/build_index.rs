use anyhow::Result;
use std::path::PathBuf;
use ragit_index_types::index_struct::Index;
use super::constants::{PROMPTS_DIR_NAME, SUMMARIZE_PROMPT_FILE_NAME};
use ragit_memory_monitor::MemoryMonitor;
use ragit_types::build_config::BuildConfig;
use text_splitter::{TextSplitter, Characters};
use crate::bootstrap_commands::build_index_logic::process_file::process_staged_file::process_staged_file;

pub mod process_file;

pub fn build_index(
    temp_dir: &PathBuf,
    _actual_root_dir: &PathBuf,
    index: &mut Index,
    _max_iterations: Option<usize>,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
) -> Result<(), anyhow::Error> {
    memory_monitor.verbose("bootstrap_index_self: Running rag build");
    memory_monitor.verbose("bootstrap_index_self: Before ragit_index_effects::build (placeholder)");
    memory_monitor.capture_and_log_snapshot("Before ragit_index_effects::build");
    memory_monitor.check_memory_limit(max_memory_gb, "Before ragit_index_effects::build")?;

    let summary_prompt_path = temp_dir.join(PROMPTS_DIR_NAME).join(SUMMARIZE_PROMPT_FILE_NAME);
    if !summary_prompt_path.exists() {
        memory_monitor.verbose("bootstrap_index_self: Warning: summarize.pdl not found, skipping prompt loading.");
    }

    let build_config = BuildConfig::default();
    let splitter = TextSplitter::new(Characters);

    memory_monitor.verbose("bootstrap_index_self: Iterating through staged files for chunking and indexing.");
    let staged_files_cloned = index.staged_files.clone();
    for file_path_buf in &staged_files_cloned {
        process_staged_file(file_path_buf, &splitter, &build_config, index, memory_monitor)?;
    }

    memory_monitor.verbose("bootstrap_index_self: After ragit_index_effects::build (placeholder)");
    memory_monitor.verbose("bootstrap_index_self: Built index (placeholder)");
    memory_monitor.capture_and_log_snapshot("After ragit_index_effects::build");
    memory_monitor.check_memory_limit(max_memory_gb, "After ragit_index_effects::build")?;
    Ok(())
}