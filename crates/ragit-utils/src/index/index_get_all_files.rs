use crate::constant::{CHUNK_DIR_NAME, FILE_INDEX_DIR_NAME, IMAGE_DIR_NAME, INDEX_DIR_NAME};
use crate::error::Error;
use crate::index::index_struct::Index;
use crate::path_utils::join3_paths;
use ragit_fs::{extension, is_dir, read_dir};
use std::path::PathBuf;

impl Index {
    fn get_files_from_index_subdir(
        &self,
        subdir_name: &str,
        extension_filter: Option<&str>,
    ) -> Result<Vec<PathBuf>, Error> {
        let mut result = vec![];
        let base_path = join3_paths(
            &self.root_dir,
            &PathBuf::from(INDEX_DIR_NAME),
            &PathBuf::from(subdir_name),
        )?;

        for internal in read_dir(base_path.to_str().unwrap(), false)? {
            if !is_dir(&internal) {
                continue;
            }

            for file_path in read_dir(&internal, false)? {
                if let Some(ext_filter) = extension_filter {
                    if let Ok(Some(ext)) = extension(&file_path) {
                        if ext == ext_filter {
                            result.push(file_path.into());
                        }
                    }
                } else {
                    result.push(file_path.into());
                }
            }
        }

        result.sort();
        Ok(result)
    }

    pub fn get_all_chunk_files(&self) -> Result<Vec<PathBuf>, Error> {
        self.get_files_from_index_subdir(CHUNK_DIR_NAME, Some("chunk"))
    }

    pub fn get_all_tfidf_files(&self) -> Result<Vec<PathBuf>, Error> {
        self.get_files_from_index_subdir(CHUNK_DIR_NAME, Some("tfidf"))
    }

    pub fn get_all_image_files(&self) -> Result<Vec<PathBuf>, Error> {
        self.get_files_from_index_subdir(IMAGE_DIR_NAME, Some("png"))
    }

    pub(crate) fn get_all_file_indexes(&self) -> Result<Vec<PathBuf>, Error> {
        self.get_files_from_index_subdir(FILE_INDEX_DIR_NAME, None)
    }
}
