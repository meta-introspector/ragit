use crate::constant::INDEX_DIR_NAME;
use crate::error::Error;
use crate::path_utils::{join_paths, str_to_pathbuf};
use ragit_fs::exists;
use std::path::PathBuf;

use crate::index::{index_struct::Index, load_mode::LoadMode};

impl Index {
    pub fn load_or_init(root_dir: PathBuf) -> Result<Self, Error> {
        let index_dir = join_paths(&root_dir, &str_to_pathbuf(INDEX_DIR_NAME))?;

        if exists(&index_dir) {
            // `load_or_init` cannot be done in only-json mode, because only-json `init` doesn't make sense
            Index::load(root_dir, LoadMode::QuickCheck)
        } else {
            Index::new(root_dir)
        }
    }
}
