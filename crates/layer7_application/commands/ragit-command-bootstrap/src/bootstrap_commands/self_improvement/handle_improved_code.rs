use anyhow::Result;
use std::path::PathBuf;
use ragit_fs::{write_string, WriteMode};
use super::super::constants::{IMPROVED_LIB_FILE_NAME, LOG_SUCCESS_WRITE_IMPROVED_SELF, LOG_EMPTY_SELF_IMPROVEMENT_RESPONSE};

pub fn handle_improved_code(
    verbose: bool,
    temp_dir: &PathBuf,
    improved_code: &str,
) -> Result<(), anyhow::Error> {
    if !improved_code.is_empty() {
        let improved_self_path = temp_dir.join(IMPROVED_LIB_FILE_NAME);
        write_string(improved_self_path.to_str().unwrap(), improved_code, WriteMode::CreateOrTruncate)?;
        if verbose {
            println!("{} {:?}", LOG_SUCCESS_WRITE_IMPROVED_SELF, improved_self_path);
        }
    } else {
        if verbose {
            println!("{}", LOG_EMPTY_SELF_IMPROVEMENT_RESPONSE);
        }
    }
    Ok(())
}