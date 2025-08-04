use anyhow::Result;
use std::path::PathBuf;

use ragit_index_types::index_struct::Index;
use super::file_source::{FileSource, CargoPackageFileSource, GlobFileSource}; // Import GlobFileSource

use super::constants::BOOTSTRAP_PACKAGE_NAME;
use ragit_memory_monitor::MemoryMonitor;

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

pub async fn add_bootstrap_files(
    actual_root_dir: &PathBuf,
    
    index: &mut Index,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
    max_files_to_process: Option<usize>,
    target: Option<String>,
) -> Result<(), anyhow::Error> {
    memory_monitor.verbose("bootstrap_index_self: Running rag add");
    memory_monitor.verbose(&format!("bootstrap_index_self: Found project root: {:?}", actual_root_dir));

    let files_to_add_source: Box<dyn FileSource> = match target.as_deref() {
        Some("all") => {
            memory_monitor.verbose("Using GlobFileSource for all Rust files.");
            Box::new(GlobFileSource {
                pattern: "**/*.rs".to_string(),
            })
        },
        _ => {
            memory_monitor.verbose("Using CargoPackageFileSource.");
            Box::new(CargoPackageFileSource {
                package_name: BOOTSTRAP_PACKAGE_NAME.to_string(),
                project_root: actual_root_dir.to_str().unwrap().to_string(),
            })
        }
    };

    let mut files_to_add = files_to_add_source.get_files()?;

    if let Some(max_files) = max_files_to_process {
        if files_to_add.len() > max_files {
            files_to_add.truncate(max_files);
        }
    }
    memory_monitor.verbose(&format!("bootstrap_index_self: Found {} files to add", files_to_add.len()));
    let absolute_files_to_add: Vec<String> = files_to_add.into_iter().filter_map(|p| {
        let abs_path = actual_root_dir.join(&p);
        if abs_path.is_file() {
            Some(abs_path.to_string_lossy().into_owned())
        } else {
            None
        }
    }).collect();

    memory_monitor.verbose("bootstrap_index_self: Before add_files_command");
    memory_monitor.capture_and_log_snapshot("Before add_files_command");
    memory_monitor.check_memory_limit(max_memory_gb, "Before add_files_command")?;
    add_files_sync(index, &absolute_files_to_add, None, false)?;
    memory_monitor.verbose("bootstrap_index_self: After add_files_command");
    memory_monitor.verbose("bootstrap_index_self: Added files to index");
    memory_monitor.capture_and_log_snapshot("After add_files_command");
    memory_monitor.check_memory_limit(max_memory_gb, "After add_files_command")?;
    Ok(())
}