use anyhow::Result;
use std::path::PathBuf;
use sysinfo::System;
use ragit_index_types::index_struct::Index;
use super::self_improvement::log_start::log_start;
use super::self_improvement::get_self_code::get_self_code;
use super::self_improvement::format_prompt::format_prompt;
use super::self_improvement::execute_query::execute_query;
use super::self_improvement::handle_improved_code::handle_improved_code;

use crate::bootstrap_commands::memory_utils::check_memory_limit;

pub async fn perform_self_improvement(
    verbose: bool,
    actual_root_dir: &PathBuf,
    temp_dir: &PathBuf,
    index: &Index,
    sys: &mut System,
    max_memory_gb: Option<u64>,
    last_process_memory_kb: &mut Option<u64>,
) -> Result<(), anyhow::Error> {
    log_start(verbose, sys, last_process_memory_kb);
    check_memory_limit(sys, max_memory_gb, "Before get_self_code")?;
    let self_code = get_self_code(actual_root_dir)?;
    check_memory_limit(sys, max_memory_gb, "After get_self_code")?;
    let prompt = format_prompt(&self_code);
    check_memory_limit(sys, max_memory_gb, "Before execute_query (self-improvement)")?;
    let improved_code = execute_query(verbose, index, &prompt, sys, max_memory_gb, last_process_memory_kb).await?;
    check_memory_limit(sys, max_memory_gb, "After execute_query (self-improvement)")?;
    handle_improved_code(verbose, temp_dir, &improved_code)?;
    check_memory_limit(sys, max_memory_gb, "After handle_improved_code")?;
    Ok(())
}