use ragit_index_types::index_struct::Index;
use ragit_types::ApiError;
use ragit_types::add_mode::{AddMode, AddResult};
use anyhow::Result;
use std::path::PathBuf;

pub async fn add_files_command(
    index: &mut Index,
    files: &[String],
    _add_mode: Option<AddMode>,
    _dry_run: bool,
) -> Result<AddResult, ApiError> {
    println!("DEBUG: Entering add_files_command. Staged files before: {}", index.staged_files.len());
    let mut added_files = 0;
    for file in files {
        let path = PathBuf::from(file);
        if !index.staged_files.contains(&path) {
            index.staged_files.push(path.clone());
            added_files += 1;
            println!("DEBUG: Added file to staged_files: {:?}", path);
        }
    }
    println!("DEBUG: Exiting add_files_command. Staged files after: {}", index.staged_files.len());

    Ok(AddResult {
        added_files,
        added_chunks: 0,
    })
}
