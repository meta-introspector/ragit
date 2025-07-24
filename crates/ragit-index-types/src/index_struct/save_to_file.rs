use crate::prelude::*;
use crate::index_struct::Index;
use ragit_fs::{write_string, WriteMode};
use anyhow::Result;

impl Index {
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        write_string(
            path,
            &serde_json::to_string_pretty(self)?,
            WriteMode::CreateOrTruncate,
        )?;

        Ok(())
    }
}
