use std::path::PathBuf;
use crate::prelude::*;
use ragit_fs::{exists, join};

use crate::index::{index_struct::Index, load_mode::LoadMode};

impl Index {
    pub fn load_or_init(
    root_dir: PathBuf,
) -> Result<Self, Error> {
        let index_dir = join(
            root_dir.to_str().unwrap(),
            &INDEX_DIR_NAME.to_string(),
        )?;

        if exists(&index_dir) {
            // `load_or_init` cannot be done in only-json mode, because only-json `init` doesn't make sense
            Index::load(root_dir, LoadMode::QuickCheck)
        }

        else {
            Index::new(root_dir)
        }
    }
}
