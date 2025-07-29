use anyhow::Result;
use sysinfo::System;
use ragit_index_types::index_struct::Index;
use ragit_utils::memory_utils::{print_memory_usage, check_memory_limit};
use super::super::constants::{FINAL_REFLECTIVE_QUERY_PROMPT, LOG_AFTER_FINAL_REFLECTIVE_QUERY};

pub async fn execute_query(
    verbose: bool,
    index: &Index,
    sys: &mut System,
    max_memory_gb: Option<u64>,
    last_snapshot_data: &mut Option<(u64, u64, u64)>,
) -> Result<(), anyhow::Error> {
    check_memory_limit(sys, max_memory_gb, "Before final reflective query")?;
    let response = ragit_index_query::query(index, FINAL_REFLECTIVE_QUERY_PROMPT, vec![], None).await?;
    println!("{}", response.get_message());
    if verbose {
        println!("{}", LOG_AFTER_FINAL_REFLECTIVE_QUERY);
        ragit_utils::memory_utils::print_memory_usage(sys, "After final reflective query", last_snapshot_data);
    }
    check_memory_limit(sys, max_memory_gb, "After final reflective query")?;
    Ok(())
}
