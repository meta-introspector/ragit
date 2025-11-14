use anyhow::Result;
use std::path::PathBuf;

use ragit_index_types::index_struct::Index;
use ragit_memory_monitor::MemoryMonitor;

use super::self_improvement::run_self_improvement_loop::run_self_improvement_loop;

pub async fn perform_self_improvement(
    verbose: bool,
    actual_root_dir: &PathBuf,
    temp_dir: &PathBuf,
    index: &Index,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
    max_iterations: Option<usize>,
) -> Result<(), anyhow::Error> {
    run_self_improvement_loop(
        verbose,
        actual_root_dir,
        temp_dir,
        index,
        max_memory_gb,
        memory_monitor,
        max_iterations,
    ).await
}