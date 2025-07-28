use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use sysinfo::System;
use ragit_index_types::index_struct::Index;
use ragit_index_save_to_file::save_index_to_file;
use super::constants::{TEMP_DIR_NAME, RAGIT_DIR_NAME, INDEX_FILE_NAME};

use ragit_utils::memory_utils::{print_memory_usage, check_memory_limit};

pub async fn setup_environment(
    verbose: bool,
    sys: &mut System,
    max_memory_gb: Option<u64>,
    last_process_memory_kb: &mut Option<u64>,
) -> Result<(PathBuf, PathBuf, Index), anyhow::Error> {
    let actual_root_dir = ragit_utils::project_root::find_root()?;
    let temp_dir = actual_root_dir.join(TEMP_DIR_NAME);

    fs::create_dir_all(&temp_dir)?;
    if verbose {
        println!("bootstrap_index_self: Created temporary directory: {:?}", temp_dir);
    }

    let ragit_dir = temp_dir.join(RAGIT_DIR_NAME);
    if !ragit_dir.exists() {
        fs::create_dir_all(&ragit_dir)?;
    }
    let index = Index::new(temp_dir.clone());
    let index_path = ragit_dir.join(INDEX_FILE_NAME);
    save_index_to_file(&index, index_path)?;
    if verbose {
        println!("bootstrap_index_self: Initialized new index in {:?}", temp_dir);
        print_memory_usage(sys, "After index initialization", last_process_memory_kb);
        check_memory_limit(sys, max_memory_gb, "After index initialization")?;
    }
    Ok((actual_root_dir, temp_dir, index))
}