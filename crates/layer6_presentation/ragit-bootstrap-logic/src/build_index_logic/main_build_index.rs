use anyhow::Result;
use std::path::PathBuf;
use ragit_index_types::index_struct::Index;
use ragit_memory_monitor::MemoryMonitor;

use text_splitter::{TextSplitter, Characters};
use ragit_types::build_config::BuildConfig;
use super::process_file::process_staged_file::process_staged_file;
use std::collections::HashSet;
use tokio::sync::mpsc;

pub async fn build_index(
    _temp_dir: &PathBuf,
    _actual_root_dir: &PathBuf,
    index: &mut Index,
    _max_iterations: Option<usize>,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
    mut file_receiver: mpsc::Receiver<PathBuf>,
) -> Result<(), anyhow::Error> {
    memory_monitor.verbose("build_index: Starting index build process.");
    memory_monitor.verbose("bootstrap_index_self: Running rag build");
    memory_monitor.verbose("bootstrap_index_self: Before ragit_index_effects::build (placeholder)");
    memory_monitor.capture_and_log_snapshot("Before ragit_index_effects::build");
    memory_monitor.check_memory_limit(max_memory_gb, "Before ragit_index_effects::build")?;

    memory_monitor.verbose("build_index: Getting staged files.");
    // The staged files are now received via the channel, so this call is no longer needed here.
    // let _staged_files = get_staged_files(index, memory_monitor)?;
    // memory_monitor.verbose(&format!("build_index: Retrieved {} staged files.", index.staged_files.len()));
    let build_config = BuildConfig::default();
    memory_monitor.verbose(&format!("BuildConfig chunk_size: {}", build_config.chunk_size));
    let splitter = TextSplitter::new(Characters);

    memory_monitor.verbose("bootstrap_index_self: Iterating through files for chunking and indexing.");
    memory_monitor.verbose("build_index: Starting chunk processing loop.");

    let mut seen_keywords: HashSet<String> = HashSet::new();

    while let Some(file_path_buf) = file_receiver.recv().await {
        process_staged_file(&file_path_buf, &splitter, &build_config, index, memory_monitor, &mut seen_keywords)?;
    }
    memory_monitor.verbose("build_index: Finished chunk processing loop.");

    memory_monitor.verbose("bootstrap_index_self: After ragit_index_effects::build (placeholder)");
    memory_monitor.verbose("bootstrap_index_self: Built index (placeholder)");
    memory_monitor.capture_and_log_snapshot("After ragit_index_effects::build");
    memory_monitor.check_memory_limit(max_memory_gb, "After ragit_index_effects::build")?;
    memory_monitor.verbose("build_index: Index build process completed.");
    Ok(())
}
