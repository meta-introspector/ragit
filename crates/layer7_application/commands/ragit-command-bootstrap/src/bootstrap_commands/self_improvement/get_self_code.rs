use anyhow::Result;
use std::path::PathBuf;
use ragit_fs::read_string;
use super::super::constants::SELF_LIB_PATH;

pub fn get_self_code(actual_root_dir: &PathBuf, memory_monitor: &mut ragit_memory_monitor::MemoryMonitor) -> Result<String, anyhow::Error> {
    memory_monitor.verbose("get_self_code: Starting to retrieve self code.");
    let self_path = actual_root_dir.join(SELF_LIB_PATH);
    memory_monitor.verbose(&format!("get_self_code: Reading from path: {:?}", self_path));
    let self_code = read_string(self_path.to_str().ok_or_else(|| anyhow::anyhow!("Invalid path for self_code: {:?}", self_path))?)?;
    memory_monitor.verbose("get_self_code: Finished retrieving self code.");
    Ok(self_code)
}
