use anyhow::Result;
use std::path::PathBuf;
use sysinfo::System;
use ragit_index_types::index_struct::Index;
use super::self_improvement::log_start::log_start;
use super::self_improvement::get_self_code::get_self_code;
use super::self_improvement::format_prompt::format_prompt;
use super::self_improvement::execute_query::execute_query;
use super::self_improvement::handle_improved_code::handle_improved_code;

pub async fn perform_self_improvement(
    verbose: bool,
    actual_root_dir: &PathBuf,
    temp_dir: &PathBuf,
    index: &Index,
    sys: &mut System,
) -> Result<(), anyhow::Error> {
    log_start(verbose, sys);
    let self_code = get_self_code(actual_root_dir)?;
    let prompt = format_prompt(&self_code);
    let improved_code = execute_query(verbose, index, &prompt, sys).await?;
    handle_improved_code(verbose, temp_dir, &improved_code)?;
    Ok(())
}