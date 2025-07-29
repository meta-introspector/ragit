use anyhow::Result;

use ragit_index_types::index_struct::Index;
use super::final_reflective_query::execute_query::execute_query;
use ragit_memory_monitor::MemoryMonitor;

pub async fn perform_final_reflective_query(
    verbose: bool,
    index: &Index,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
) -> Result<(), anyhow::Error> {
    memory_monitor.check_memory_limit(max_memory_gb, "Before execute_query (final reflective query)")?;
    execute_query(verbose, index, &mut memory_monitor.sys, max_memory_gb, &mut memory_monitor.last_snapshot_data).await?;
    memory_monitor.check_memory_limit(max_memory_gb, "After execute_query (final reflective query)")?;
    Ok(())
}
