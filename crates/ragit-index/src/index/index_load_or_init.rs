use crate::prelude::*;

impl Index {
    pub fn load_or_init(root_dir: PathBuf) -> Result<Self, Error> {
        let index_dir = join_paths(&root_dir, &PathBuf::from(INDEX_DIR_NAME))?;

        if exists(&index_dir) {
            // `load_or_init` cannot be done in only-json mode, because only-json `init` doesn't make sense
            Index::load(root_dir, LoadMode::QuickCheck)
        } else {
            Index::new(root_dir)
        }
    }
}
