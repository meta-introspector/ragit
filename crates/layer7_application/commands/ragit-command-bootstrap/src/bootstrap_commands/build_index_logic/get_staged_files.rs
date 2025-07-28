use anyhow::Result;
use std::path::PathBuf;
use ragit_index_types::index_struct::Index;
use ragit_index_storage;

pub fn get_staged_files(
    index: &Index,
) -> Result<Vec<PathBuf>, anyhow::Error> {
    let staged_files = ragit_index_storage::get_all_staged_files(&index.root_dir)?;
    Ok(staged_files)
}