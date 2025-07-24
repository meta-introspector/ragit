use ragit_types::prelude::*;
use ragit_types::Index;
use ragit_fs::{write_string, WriteMode};
use anyhow::Result;
use std::path::PathBuf;

pub fn save_index_to_file(index: &Index, path: PathBuf) -> Result<()> {
    write_string(
        path.to_str().unwrap(),
        &serde_json::to_string_pretty(index)?,
        WriteMode::CreateOrTruncate,
    )?;

    Ok(())
}
