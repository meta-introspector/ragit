use anyhow::Result;
use std::path::PathBuf;
use sysinfo::System;
use ragit_index_core::add_files::add_files_command;
use ragit_index_types::index_struct::Index;
use crate::file_source::FileSource;
use crate::file_copy_utils;
use super::constants::BOOTSTRAP_PACKAGE_NAME;

use ragit_utils::memory_utils::{print_memory_usage, check_memory_limit};

pub async fn add_bootstrap_files(
    verbose: bool,
    actual_root_dir: &PathBuf,
    temp_dir: &PathBuf,
    index: &mut Index,
    sys: &mut System,
    max_memory_gb: Option<u64>,
    last_snapshot_data: &mut Option<(u64, u64, u64)>,
    max_files_to_process: Option<usize>,
) -> Result<(), anyhow::Error> {
    if verbose {
        println!("bootstrap_index_self: Running rag add");
    }
    if verbose {
        println!("bootstrap_index_self: Found project root: {:?}", actual_root_dir);
    }
    let bootstrap_source = crate::file_source::CargoPackageFileSource { 
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
    ).await?;

    let relative_temp_files_to_add = temp_files_to_add.iter().map(|p| {
        p.strip_prefix(&temp_dir).unwrap().to_string_lossy().into_owned()
    }).collect::<Vec<String>>();
    if verbose {
        println!("bootstrap_index_self: Before add_files_command");
        print_memory_usage(sys, "Before add_files_command", last_snapshot_data);
    }
    check_memory_limit(sys, max_memory_gb, "Before add_files_command")?;
    add_files_command(index, &relative_temp_files_to_add, None, false).await?;
    if verbose {
        println!("bootstrap_index_self: After add_files_command");
        println!("bootstrap_index_self: Added files to index");
        print_memory_usage(sys, "After add_files_command", last_snapshot_data);
    }
    check_memory_limit(sys, max_memory_gb, "After add_files_command")?;
    Ok(())
}