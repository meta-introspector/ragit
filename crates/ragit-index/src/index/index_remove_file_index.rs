use crate::prelude::*;

impl Index {
    pub fn remove_file_index(&mut self, file_uid: Uid) -> Result<(), ApiError> {
        let file_index_path = get_uid_path(&self.root_dir, &Path::new(FILE_INDEX_DIR_NAME), file_uid, None)?;

        if !exists(&file_index_path) {
            return Err(ApiError::NoSuchFile {
                file: file_index_path.to_string_lossy().to_string(),
                similar_files: vec![],
            });
        }

        remove_file(&file_index_path)?;

        Ok(())
    }
}