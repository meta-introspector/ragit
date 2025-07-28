use anyhow::Result;
use std::path::PathBuf;
use sysinfo::System;
use ragit_index_types::index_struct::Index;
use super::file_source::FileSource;
use super::file_copy_utils;
use super::constants::BOOTSTRAP_PACKAGE_NAME;

use ragit_utils::memory_utils::{print_memory_usage, check_memory_limit};

// Define AddResult and AddMode locally for synchronous operation
pub struct AddResult {
    pub added_files: usize,
    pub added_chunks: usize,
}

pub enum AddMode {
    // Define variants if needed, otherwise keep empty
}

// Synchronous version of add_files_command
pub fn add_files_sync(
    index: &mut Index,
    files: &[String],
    _add_mode: Option<AddMode>,
    _dry_run: bool,
) -> Result<AddResult, anyhow::Error> {
    let mut added_files = 0;
    for file in files {
        let path = PathBuf::from(file);
        if !index.staged_files.contains(&path) {
            index.staged_files.push(path);
            added_files += 1;
        }
    }

    Ok(AddResult {
        added_files,
        added_chunks: 0,
    })
}

pub fn add_bootstrap_files(
    verbose: bool,
    actual_root_dir: &PathBuf,
    temp_dir: &PathBuf,
    index: &mut Index,
    sys: &mut System,
    max_memory_gb: Option<u64>,
    last_process_memory_kb: &mut Option<u64>,
    max_files_to_process: Option<usize>,
) -> Result<(), anyhow::Error> {
    if verbose {
        println!("bootstrap_index_self: Running rag add");
    }
    if verbose {
        println!("bootstrap_index_self: Found project root: {:?}", actual_root_dir);
    }
    let bootstrap_source = super::file_source::CargoPackageFileSource { 
        package_name: BOOTSTRAP_PACKAGE_NAME.to_string(),
        project_root: actual_root_dir.to_str().unwrap().to_string(),
    };
    let mut original_files_to_add = bootstrap_source.get_files()?;
    if let Some(max_files) = max_files_to_process {
        if original_files_to_add.len() > max_files {
            original_files_to_add.truncate(max_files);
        }
    }
    if verbose {
        println!("bootstrap_index_self: Found {} files to add", original_files_to_add.len());
    }
    let temp_files_to_add = file_copy_utils::copy_files_to_temp_dir(
        &actual_root_dir,
        &temp_dir,
        original_files_to_add,
        verbose,
    )?;

    let relative_temp_files_to_add = temp_files_to_add.iter().map(|p| {
        p.strip_prefix(&temp_dir).unwrap().to_string_lossy().into_owned()
    }).collect::<Vec<String>>();
    if verbose {
        println!("bootstrap_index_self: Before add_files_command");
        print_memory_usage(sys, "Before add_files_command", last_process_memory_kb);
    }
    check_memory_limit(sys, max_memory_gb, "Before add_files_command")?;
    add_files_sync(index, &relative_temp_files_to_add, None, false)?;
    if verbose {
        println!("bootstrap_index_self: After add_files_command");
        println!("bootstrap_index_self: Added files to index");
        print_memory_usage(sys, "After add_files_command", last_process_memory_kb);
    }
    check_memory_limit(sys, max_memory_gb, "After add_files_command")?;
    Ok(())
}