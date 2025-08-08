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
    let total_files = files.len();
    let mut next_percentage_log = 10;

    for (i, file) in files.iter().enumerate() {
        let path = PathBuf::from(file);
        if !index.staged_files.contains(&path) {
            index.staged_files.push(path.clone());
            added_files += 1;
        }

        let current_percentage = ((i + 1) as f64 / total_files as f64 * 100.0) as usize;

        if current_percentage >= next_percentage_log {
            println!("DEBUG: Adding files to index: {}% complete.", next_percentage_log);
            next_percentage_log += 10;
        }
    }
    println!("DEBUG: Exiting add_files_command. Staged files after: {}", index.staged_files.len());

    Ok(AddResult {
        added_files,
        added_chunks: 0,
    })
}
