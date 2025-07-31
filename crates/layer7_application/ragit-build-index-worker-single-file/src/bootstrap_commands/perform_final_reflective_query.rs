use anyhow::Result;

use ragit_index_types::index_struct::Index;
use crate::bootstrap_commands::final_reflective_query::execute_query::execute_query;
use ragit_memory_monitor::MemoryMonitor;

pub async fn perform_final_reflective_query(
    verbose: bool,
    index: &Index,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
) -> Result<(), anyhow::Error> {
    memory_monitor.check_memory_limit(max_memory_gb, "Before execute_query (final reflective query)")?;
    execute_query(verbose, index, max_memory_gb, memory_monitor).await?;
    memory_monitor.check_memory_limit(max_memory_gb, "After execute_query (final reflective query)")?;
    Ok(())
}
