use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::env;

use ragit_index_types::index_struct::Index;
use ragit_index_save_to_file::save_index_to_file;
use super::constants::{TEMP_DIR_NAME, RAGIT_DIR_NAME, INDEX_FILE_NAME};



use ragit_memory_monitor::MemoryMonitor;

pub async fn setup_environment(
    _max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
) -> Result<(PathBuf, PathBuf, Index), anyhow::Error> {
    let actual_root_dir = env::current_dir()?;
    let temp_dir = actual_root_dir.join(TEMP_DIR_NAME);

    fs::create_dir_all(&temp_dir)?;
    memory_monitor.verbose(&format!("bootstrap_index_self: Created temporary directory: {:?}", temp_dir));

    let ragit_dir = temp_dir.join(RAGIT_DIR_NAME);
    memory_monitor.verbose(&format!("setup_environment: Attempting to create .ragit directory: {:?}", ragit_dir));
    if !ragit_dir.exists() {
        fs::create_dir_all(&ragit_dir)?;
        memory_monitor.verbose("setup_environment: Created .ragit directory.");
    } else {
        memory_monitor.verbose("setup_environment: .ragit directory already exists.");
    }
    let index = Index::new(temp_dir.clone());
    memory_monitor.verbose("setup_environment: Initialized new Index struct.");
    let index_path = ragit_dir.join(INDEX_FILE_NAME);
    memory_monitor.verbose(&format!("setup_environment: Index file path: {:?}", index_path));
    match save_index_to_file(&index, index_path.clone()) {
        Ok(_) => memory_monitor.verbose(&format!("setup_environment: Successfully saved index to {:?}", index_path)),
        Err(e) => memory_monitor.verbose(&format!("setup_environment: Failed to save index to {:?}: {:?}", index_path, e)),
    }
    memory_monitor.verbose(&format!("bootstrap_index_self: Initialized new index in {:?}", temp_dir));
    memory_monitor.capture_and_log_snapshot("After index initialization");
    Ok((actual_root_dir, temp_dir, index))
}


