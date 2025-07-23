use crate::constant::FILE_INDEX_DIR_NAME;
use crate::error::Error;
use crate::path_utils::get_uid_path;
use ragit_fs::{exists, parent, try_create_dir};
use ragit_uid::{Uid, UidWriteMode};
use std::path::{Path, PathBuf};
use crate::index::index_struct::Index;
impl Index {
    pub fn add_file_index(&mut self, file_uid: Uid, uids: &[Uid]) -> Result<(), Error> {
        let file_index_path = get_uid_path(&self.root_dir, &Path::new(FILE_INDEX_DIR_NAME), file_uid, None)?;;
        let parent_path = parent(&file_index_path)?;

        if !exists(&parent_path) {
            try_create_dir(parent_path.to_str().unwrap())?;
        }

        Ok(ragit_uid::save_to_file(
            &file_index_path,
            &uids.to_vec(),
            UidWriteMode::Naive,
        )?)
    }
}
