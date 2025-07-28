use anyhow::Result;
use std::path::PathBuf;
use ragit_fs::read_string;
use super::super::constants::SELF_LIB_PATH;

pub fn get_self_code(actual_root_dir: &PathBuf) -> Result<String, anyhow::Error> {
    let self_path = actual_root_dir.join(SELF_LIB_PATH);
    let self_code = read_string(self_path.to_str().ok_or_else(|| anyhow::anyhow!("Invalid path for self_code: {:?}", self_path))?)?;
    Ok(self_code)
}
