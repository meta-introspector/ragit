use crate::prelude::*;
use ragit_fs::{write_bytes, WriteMode};
use std::path::PathBuf;

use crate::index::index_struct::Index;

impl Index {
    pub fn save_to_file(&self, path: PathBuf) -> Result<(), Error> {
        self.save_prompts()?;

        Ok(write_bytes(
            path.to_str().unwrap(),
            &serde_json::to_vec_pretty(self)?,
            WriteMode::Atomic,
        )?)
    }
}
