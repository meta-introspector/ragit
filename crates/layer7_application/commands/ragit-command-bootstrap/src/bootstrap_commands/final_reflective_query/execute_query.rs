use anyhow::Result;

use ragit_index_types::index_struct::Index;
use memory_profiler::memory_monitor::MemoryMonitor;
use super::super::constants::{FINAL_REFLECTIVE_QUERY_PROMPT, LOG_AFTER_FINAL_REFLECTIVE_QUERY};

pub async fn execute_query(
    verbose: bool,
    index: &Index,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
) -> Result<(), anyhow::Error> {
    memory_monitor.check_memory_limit(max_memory_gb, "Before final reflective query")?;
    let response = ragit_index_query::query(index, FINAL_REFLECTIVE_QUERY_PROMPT, vec![], None).await?;
    println!("{}", response.get_message());
    if verbose {
        println!("{}", LOG_AFTER_FINAL_REFLECTIVE_QUERY);
        memory_monitor.capture_and_log_snapshot("After final reflective query");
    }
    memory_monitor.check_memory_limit(max_memory_gb, "After final reflective query")?;
    Ok(())
}
