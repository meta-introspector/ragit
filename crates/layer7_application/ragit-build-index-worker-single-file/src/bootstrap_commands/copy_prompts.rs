use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use sysinfo::System;
use super::constants::PROMPTS_DIR_NAME;

use ragit_utils::memory_utils::{print_memory_usage, check_memory_limit};

pub fn copy_prompts(
    verbose: bool,
    actual_root_dir: &PathBuf,
    temp_dir: &PathBuf,
    sys: &mut System,
    max_memory_gb: Option<u64>,
    last_snapshot_data: &mut Option<(u64, u64, u64)>,
) -> Result<(), anyhow::Error> {
    if verbose {
        println!("bootstrap_index_self: Copying prompts");
        print_memory_usage(sys, "Before copy_prompts", last_snapshot_data);
    }
    check_memory_limit(sys, max_memory_gb, "Before copy_prompts")?;
    let prompts_dir = actual_root_dir.join(PROMPTS_DIR_NAME);
    let temp_prompts_dir = temp_dir.join(PROMPTS_DIR_NAME);
    fs::create_dir_all(&temp_prompts_dir)?;
    for entry in fs::read_dir(prompts_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let dest_path = temp_prompts_dir.join(path.file_name().unwrap());
            fs::copy(&path, &dest_path)?;
            if verbose {
                println!(r#"bootstrap_index_self: Copied prompt {:?} to {:?}"#, path, dest_path);
            }
            if path.file_name().unwrap() == "summarize.pdl" {
                println!("DEBUG: summarize.pdl copied to: {:?}", dest_path);
            }
        }
        check_memory_limit(sys, max_memory_gb, &format!("During copy_prompts loop for {:?}", path))?;
    }
    if verbose {
        print_memory_usage(sys, "After copy_prompts", last_process_memory_kb);
    }
    Ok(())
}