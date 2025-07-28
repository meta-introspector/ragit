use anyhow::Result;
use sysinfo::System;
use ragit_index_types::index_struct::Index;
use super::final_reflective_query::log_start::log_start;
use super::final_reflective_query::execute_query::execute_query;

pub async fn perform_final_reflective_query(
    verbose: bool,
    index: &Index,
    sys: &mut System,
    max_memory_gb: Option<u64>,
    last_process_memory_kb: Option<&mut u64>,
) -> Result<(), anyhow::Error> {
    log_start(verbose, sys, last_process_memory_kb);
    check_memory_limit(sys, max_memory_gb, "Before execute_query (final reflective query)")?;
    execute_query(verbose, index, sys, max_memory_gb, last_process_memory_kb).await?;
    check_memory_limit(sys, max_memory_gb, "After execute_query (final reflective query)")?;
    Ok(())
}