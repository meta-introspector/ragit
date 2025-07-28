use anyhow::Result;
use sysinfo::System;
use ragit_index_types::index_struct::Index;
use super::final_reflective_query::log_start::log_start;
use super::final_reflective_query::execute_query::execute_query;

pub async fn perform_final_reflective_query(
    verbose: bool,
    index: &Index,
    sys: &mut System,
) -> Result<(), anyhow::Error> {
    log_start(verbose, sys);
    execute_query(verbose, index, sys).await?;
    Ok(())
}