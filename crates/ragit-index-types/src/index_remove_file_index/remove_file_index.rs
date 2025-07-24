use crate::prelude::*;
use crate::index_struct::Index;
use ragit_utils::ragit_path_utils::get_uid_path;
use ragit_fs::{exists, remove_file};
use ragit_error::ApiError;
use ragit_types::Uid;

impl Index {
    pub fn remove_file_index(&mut self, file_uid: Uid) -> Result<(), ApiError> {
        let file_index_path = get_uid_path(&self.root_dir, FILE_INDEX_DIR_NAME, file_uid, None)?;

        if !exists(&file_index_path) {
            return Err(ApiError::NoSuchFile {
                path: Some(file_index_path.to_string_lossy().to_string()),
                uid: Some(file_uid.to_string()),
            });
        }

        remove_file(file_index_path.to_str().unwrap())?;

        Ok(())
    }
}
