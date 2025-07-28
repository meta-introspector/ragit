use anyhow::Result;
use sysinfo::System;
use ragit_index_types::index_struct::Index;
use super::super::memory_utils::print_memory_usage;

pub async fn execute_query(
    verbose: bool,
    index: &Index,
    prompt: &str,
    sys: &mut System,
) -> Result<String, anyhow::Error> {
    if verbose {
        print_memory_usage(sys, "Before self-improvement query");
    }
    let response = ragit_index_query::query(index, prompt, vec![], None).await?;
    let improved_code = response.get_message();
    if verbose {
        print_memory_usage(sys, "After self-improvement query");
    }
    Ok(improved_code.to_string())
}
