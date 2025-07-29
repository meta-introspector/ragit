use anyhow::Result;
use std::path::PathBuf;
use ragit_index_types::index_struct::Index;

pub fn get_staged_files(
    index: &Index,
) -> Result<Vec<PathBuf>, anyhow::Error> {
    Ok(index.staged_files.clone())
}