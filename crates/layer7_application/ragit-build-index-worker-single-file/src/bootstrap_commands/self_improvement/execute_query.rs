use anyhow::Result;

use ragit_index_types::index_struct::Index;
use ragit_memory_monitor::MemoryMonitor;

pub async fn execute_query(
    verbose: bool,
    index: &Index,
    prompt: &str,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
) -> Result<String, anyhow::Error> {
    if verbose {
        memory_monitor.capture_and_log_snapshot("Before self-improvement query");
    }
    memory_monitor.check_memory_limit(max_memory_gb, "Before self-improvement query")?;
    let response = ragit_index_query::query(index, prompt, vec![], None, memory_monitor).await?;
    let improved_code = response.get_message();
    if verbose {
        memory_monitor.capture_and_log_snapshot("After self-improvement query");
    }
    memory_monitor.check_memory_limit(max_memory_gb, "After self-improvement query")?;
    Ok(improved_code.to_string())
}
