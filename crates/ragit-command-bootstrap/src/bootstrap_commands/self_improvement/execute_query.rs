use anyhow::Result;
use sysinfo::System;
use ragit_index_types::index_struct::Index;
use super::super::memory_utils::{print_memory_usage, check_memory_limit};

pub async fn execute_query(
    verbose: bool,
    index: &Index,
    prompt: &str,
    sys: &mut System,
    max_memory_gb: Option<u64>,
    last_process_memory_kb: &mut Option<u64>,
) -> Result<String, anyhow::Error> {
    if verbose {
        print_memory_usage(sys, "Before self-improvement query", last_process_memory_kb);
    }
    check_memory_limit(sys, max_memory_gb, "Before self-improvement query")?;
    let response = ragit_index_query::query(index, prompt, vec![], None).await?;
    let improved_code = response.get_message();
    if verbose {
        print_memory_usage(sys, "After self-improvement query", last_process_memory_kb);
    }
    check_memory_limit(sys, max_memory_gb, "After self-improvement query")?;
    Ok(improved_code.to_string())
}
