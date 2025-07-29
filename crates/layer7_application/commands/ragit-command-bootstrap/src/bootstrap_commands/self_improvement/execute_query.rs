use anyhow::Result;
use sysinfo::System;
use ragit_index_types::index_struct::Index;
use ragit_utils::memory_utils::{print_memory_usage, check_memory_limit};

pub async fn execute_query(
    verbose: bool,
    index: &Index,
    prompt: &str,
    sys: &mut System,
    max_memory_gb: Option<u64>,
    last_snapshot_data: &mut Option<(u64, u64, u64)>,
) -> Result<String, anyhow::Error> {
    if verbose {
        ragit_utils::memory_utils::print_memory_usage(sys, "Before self-improvement query", last_snapshot_data);
    }
    check_memory_limit(sys, max_memory_gb, "Before self-improvement query")?;
    let response = ragit_index_query::query(index, prompt, vec![], None).await?;
    let improved_code = response.get_message();
    if verbose {
        ragit_utils::memory_utils::print_memory_usage(sys, "After self-improvement query", last_snapshot_data);
    }
    check_memory_limit(sys, max_memory_gb, "After self-improvement query")?;
    Ok(improved_code.to_string())
}
