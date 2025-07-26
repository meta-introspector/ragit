use ragit_index_types::index_struct::Index;
use std::path::PathBuf;

impl Index {
    pub fn get_path(&self) -> &PathBuf {
        &self.root_dir
    }
}