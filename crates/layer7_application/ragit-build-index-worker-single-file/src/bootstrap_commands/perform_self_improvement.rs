use anyhow::Result;
use std::path::PathBuf;

use ragit_index_types::index_struct::Index;
use crate::bootstrap_commands::self_improvement::log_start::log_start;
use crate::bootstrap_commands::self_improvement::get_self_code::get_self_code;
use crate::bootstrap_commands::self_improvement::format_prompt::format_prompt;
use crate::bootstrap_commands::self_improvement::execute_query::execute_query;
use crate::bootstrap_commands::self_improvement::handle_improved_code::handle_improved_code;
use ragit_memory_monitor::MemoryMonitor;

pub async fn perform_self_improvement(
    verbose: bool,
    actual_root_dir: &PathBuf,
    temp_dir: &PathBuf,
    index: &Index,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
    max_iterations: Option<usize>,
) -> Result<(), anyhow::Error> {
    for i in 0..max_iterations.unwrap_or(1) {
        memory_monitor.verbose(&format!("--- Self-Improvement Iteration {} ---", i + 1));
        log_start(verbose, memory_monitor);
        memory_monitor.check_memory_limit(max_memory_gb, "Before get_self_code")?;
        let self_code = get_self_code(actual_root_dir)?;
        memory_monitor.check_memory_limit(max_memory_gb, "After get_self_code")?;
        let prompt = format_prompt(&self_code);
        memory_monitor.check_memory_limit(max_memory_gb, "Before execute_query (self-improvement)")?;
        let improved_code = execute_query(verbose, index, &prompt, max_memory_gb, memory_monitor).await?;
        memory_monitor.check_memory_limit(max_memory_gb, "After execute_query (self-improvement)")?;
        handle_improved_code(verbose, temp_dir, &improved_code, memory_monitor)?;
        memory_monitor.check_memory_limit(max_memory_gb, "After handle_improved_code")?;
    }
    Ok(())
}
