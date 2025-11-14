use anyhow::Result;
use std::fs;
use std::path::PathBuf;

use ragit_index_types::index_struct::Index;
use ragit_index_save_to_file::save_index_to_file;
use super::constants::{TEMP_DIR_NAME, RAGIT_DIR_NAME, INDEX_FILE_NAME};
use ragit_memory_monitor::MemoryMonitor;

pub async fn setup_environment(
    _verbose: bool,
    _max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
) -> Result<(PathBuf, PathBuf, Index), anyhow::Error> {
    memory_monitor.verbose("setup_environment: Starting environment setup.");
    let actual_root_dir = ragit_utils::project_root::find_root()?;
    memory_monitor.verbose(&format!("setup_environment: Found actual root directory: {:?}", actual_root_dir));
    let temp_dir = actual_root_dir.join(TEMP_DIR_NAME);

    fs::create_dir_all(&temp_dir)?;
    memory_monitor.verbose(&format!("setup_environment: Created temporary directory: {:?}", temp_dir));

    let ragit_dir = temp_dir.join(RAGIT_DIR_NAME);
    memory_monitor.verbose(&format!("setup_environment: Ragit directory: {:?}", ragit_dir));
    if !ragit_dir.exists() {
        fs::create_dir_all(&ragit_dir)?;
        memory_monitor.verbose("setup_environment: Created .ragit directory.");
    }
    let index = Index::new(temp_dir.clone());
    memory_monitor.verbose("setup_environment: Initialized new Index struct.");
    let index_path = ragit_dir.join(INDEX_FILE_NAME);
    memory_monitor.verbose(&format!("setup_environment: Index file path: {:?}", index_path));
    save_index_to_file(&index, index_path)?;
    memory_monitor.verbose(&format!("setup_environment: Initialized new index in {:?}", temp_dir));
    memory_monitor.verbose("setup_environment: Finished environment setup.");
    Ok((actual_root_dir, temp_dir, index))
}
