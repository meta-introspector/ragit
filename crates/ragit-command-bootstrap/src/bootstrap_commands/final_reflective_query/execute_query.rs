use anyhow::Result;
use sysinfo::System;
use ragit_index_types::index_struct::Index;
use super::super::memory_utils::print_memory_usage;
use super::super::constants::{FINAL_REFLECTIVE_QUERY_PROMPT, LOG_AFTER_FINAL_REFLECTIVE_QUERY};

pub async fn execute_query(
    verbose: bool,
    index: &Index,
    sys: &mut System,
) -> Result<(), anyhow::Error> {
    let response = ragit_index_query::query(index, FINAL_REFLECTIVE_QUERY_PROMPT, vec![], None).await?;
    println!("{}", response.get_message());
    if verbose {
        println!("{}", LOG_AFTER_FINAL_REFLECTIVE_QUERY);
        print_memory_usage(sys, "After final reflective query");
    }
    Ok(())
}
