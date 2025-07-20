use std::path::PathBuf;
use crate::constant::INDEX_FILE_NAME;
use crate::error::Error;
use ragit_fs::{write_bytes, WriteMode};

use crate::index::index_struct::Index;

impl Index {
    pub fn save_to_file(&self, path: PathBuf) -> Result<(), Error> {
        self.save_prompts()?;

        Ok(write_bytes(
            &path,
            &serde_json::to_vec_pretty(self)?,
            WriteMode::Atomic,
        ))
    }
}
