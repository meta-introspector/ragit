use anyhow::Result;
use std::path::PathBuf;
use tokio::sync::mpsc;

use ragit_index_core::add_files::add_files_command;
use ragit_index_types::index_struct::Index;
use crate::file_source::FileSource;
use crate::file_copy_utils;
use ragit_memory_monitor::MemoryMonitor;
use crate::file_source::GlobFileSource;

pub async fn add_bootstrap_files(
    verbose: bool,
    actual_root_dir: &PathBuf,
    temp_dir: &PathBuf,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
    max_files_to_process: Option<usize>,
    target: Option<String>,
) -> Result<mpsc::Receiver<PathBuf>, anyhow::Error> {
    memory_monitor.verbose("add_bootstrap_files: Starting to add bootstrap files.");
    memory_monitor.verbose("Running rag add.");
    memory_monitor.verbose(&format!("Found project root: {:?}", actual_root_dir));

    let (tx, rx) = mpsc::channel(100); // Create a channel with a buffer of 100

    let glob_pattern = if let Some(target_glob) = target.clone() {
        format!("{}/{}", actual_root_dir.to_string_lossy(), target_glob)
    } else {
        format!("{}/**/*.rs", actual_root_dir.to_string_lossy())
    };
    let bootstrap_source = GlobFileSource { pattern: glob_pattern };

    let mut original_files_to_add = bootstrap_source.get_files(memory_monitor)?;
    if let Some(max_files) = max_files_to_process {
        if original_files_to_add.len() > max_files {
            original_files_to_add.truncate(max_files);
            memory_monitor.verbose(&format!("add_bootstrap_files: Truncated files to process to {} files.", max_files));
        }
    }
    memory_monitor.verbose(&format!("Found {} files to add", original_files_to_add.len()));

    let temp_dir_clone = temp_dir.clone();
    let actual_root_dir_clone = actual_root_dir.clone();
    let verbose_clone = verbose;
    let memory_monitor_clone = memory_monitor.clone(); // Clone memory_monitor for the spawned task

    tokio::spawn(async move {
        let temp_files_to_add = file_copy_utils::copy_files_to_temp_dir(
            &actual_root_dir_clone,
            &temp_dir_clone,
            original_files_to_add,
            verbose_clone,
            &mut memory_monitor_clone.clone(), // Pass a mutable clone
        ).await.expect("Failed to copy files to temp dir");

        for file_path in temp_files_to_add {
            if let Err(e) = tx.send(file_path).await {
                eprintln!("Failed to send file path through channel: {}", e);
                break;
            }
        }
        // Sender will be dropped here, signaling the end of the stream
    });

    Ok(rx)
}
