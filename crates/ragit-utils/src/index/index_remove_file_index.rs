use crate::constant::FILE_INDEX_DIR_NAME;
use crate::error::Error;
use crate::path_utils::get_uid_path;
use ragit_fs::{exists, remove_file};
use ragit_uid::Uid;
use std::path::{Path, PathBuf};

impl Index {
    pub fn remove_file_index(&mut self, file_uid: Uid) -> Result<(), Error> {
        let file_index_path = get_uid_path(&self.root_dir, &Path::new(FILE_INDEX_DIR_NAME), file_uid, None)?;

        if !exists(&file_index_path) {
            return Err(Error::NoSuchFile {
                path: None,
                uid: Some(file_uid),
            });
        }

        Ok(remove_file(file_index_path.to_str().unwrap())?)
    }
}
