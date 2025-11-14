use anyhow::Result;
use std::path::PathBuf;

use ragit_index_core::add_files::add_files_command;
use ragit_index_types::index_struct::Index;
use crate::file_source::FileSource;
use crate::file_copy_utils;
use super::constants::BOOTSTRAP_PACKAGE_NAME;
use ragit_memory_monitor::MemoryMonitor;

pub async fn add_bootstrap_files(
    verbose: bool,
    actual_root_dir: &PathBuf,
    temp_dir: &PathBuf,
    index: &mut Index,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
    max_files_to_process: Option<usize>,
) -> Result<(), anyhow::Error> {
    memory_monitor.verbose("add_bootstrap_files: Starting to add bootstrap files.");
    memory_monitor.verbose("Running rag add.");
    memory_monitor.verbose(&format!("Found project root: {:?}", actual_root_dir));
    let bootstrap_source = crate::file_source::CargoPackageFileSource { 
        package_name: BOOTSTRAP_PACKAGE_NAME.to_string(),
        project_root: actual_root_dir.to_str().unwrap().to_string(),
    };
    let mut original_files_to_add = bootstrap_source.get_files(memory_monitor)?;
    if let Some(max_files) = max_files_to_process {
        if original_files_to_add.len() > max_files {
            original_files_to_add.truncate(max_files);
            memory_monitor.verbose(&format!("add_bootstrap_files: Truncated files to process to {} files.", max_files));
        }
    }
    memory_monitor.verbose(&format!("Found {} files to add", original_files_to_add.len()));
    let temp_files_to_add = file_copy_utils::copy_files_to_temp_dir(
        &actual_root_dir,
        &temp_dir,
        original_files_to_add,
        verbose,
        memory_monitor,
    ).await?;

    let relative_temp_files_to_add = temp_files_to_add.iter().map(|p| {
        p.strip_prefix(&temp_dir).unwrap().to_string_lossy().into_owned()
    }).collect::<Vec<String>>();
    memory_monitor.verbose(&format!("Chunks in index before add_files_command: {}", index.chunks.len()));
    memory_monitor.verbose("Adding files to index.");
    memory_monitor.capture_and_log_snapshot("Before add_files_command");
    memory_monitor.check_memory_limit(max_memory_gb, "Before add_files_command")?;
    add_files_command(index, &relative_temp_files_to_add, None, false).await?;
    memory_monitor.verbose("Files added to index.");
    memory_monitor.capture_and_log_snapshot("After add_files_command");
    memory_monitor.verbose(&format!("Chunks in index after add_files_command: {}", index.chunks.len()));
    memory_monitor.check_memory_limit(max_memory_gb, "After add_files_command")?;
    memory_monitor.verbose("add_bootstrap_files: Finished adding bootstrap files.");
    Ok(())
}
