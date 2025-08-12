use anyhow::Result;
use std::path::PathBuf;
use ragit_index_types::index_struct::Index;

use ragit_memory_monitor::MemoryMonitor;
pub fn get_staged_files(
    index: &Index,
    memory_monitor: &mut MemoryMonitor,
) -> Result<Vec<PathBuf>, anyhow::Error> {
    memory_monitor.verbose("get_staged_files: Starting to retrieve staged files.");
    let staged_files = index.staged_files.clone();
    memory_monitor.verbose(&format!("get_staged_files: Found {} staged files.", staged_files.len()));
    memory_monitor.verbose("get_staged_files: Finished retrieving staged files.");
    Ok(staged_files)
}