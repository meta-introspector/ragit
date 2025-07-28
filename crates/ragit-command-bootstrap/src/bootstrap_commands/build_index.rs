use anyhow::Result;
use std::path::PathBuf;
use sysinfo::System;
use ragit_index_types::index_struct::Index;
use super::constants::{PROMPTS_DIR_NAME, SUMMARIZE_PROMPT_FILE_NAME};

use crate::bootstrap_commands::memory_utils::print_memory_usage;

pub async fn build_index(
    verbose: bool,
    temp_dir: &PathBuf,
    index: &mut Index,
    max_iterations: Option<usize>,
    sys: &mut System,
) -> Result<(), anyhow::Error> {
    if verbose {
        println!("bootstrap_index_self: Running rag build");
        println!("bootstrap_index_self: Before ragit_index_effects::build");
        print_memory_usage(sys, "Before ragit_index_effects::build");
    }

    let summary_prompt_path = temp_dir.join(PROMPTS_DIR_NAME).join(SUMMARIZE_PROMPT_FILE_NAME);
    if !summary_prompt_path.exists() {
        return Err(anyhow::anyhow!("summarize.pdl not found in temporary directory!"));
    }

    ragit_index_effects::build(index, 1, false, true, max_iterations).await?;
    if verbose {
        println!("bootstrap_index_self: After ragit_index_effects::build");
        println!("bootstrap_index_self: Built index");
        print_memory_usage(sys, "After ragit_index_effects::build");
    }
    Ok(())
}