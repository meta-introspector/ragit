use ragit_index_types::index_struct::Index;
use ragit_types::ApiError;
use ragit_types::add_mode::{AddMode, AddResult};
use anyhow::Result;
use std::path::PathBuf;

pub async fn add_files_command(
    index: &mut Index,
    files: &[String],
    add_mode: Option<AddMode>,
    dry_run: bool,
) -> Result<AddResult, ApiError> {
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