use crate::constant::{CHUNK_DIR_NAME, FILE_INDEX_DIR_NAME, IMAGE_DIR_NAME, INDEX_DIR_NAME};
use crate::error::Error;
use ragit_fs::{extension, is_dir, join3, read_dir};

use crate::Path;
use crate::index::index_struct::Index;

impl Index {
    pub fn get_all_chunk_files(&self) -> Result<Vec<Path>, Error> {
        let mut result = vec![];

        for internal in read_dir(&join3(&self.root_dir, &INDEX_DIR_NAME, &CHUNK_DIR_NAME)?, false)? {
            if !is_dir(&internal) {
                continue;
            }

            for chunk_file in read_dir(&internal, false)? {
                if extension(&chunk_file).unwrap_or(None).unwrap_or(String::new()) == "chunk" {
                    result.push(chunk_file.to_string());
                }
            }
        }

        // the result has to be deterministic
        result.sort();
        Ok(result)
    }

    pub fn get_all_tfidf_files(&self) -> Result<Vec<Path>, Error> {
        let mut result = vec![];

        for internal in read_dir(&join3(&self.root_dir, &INDEX_DIR_NAME, &CHUNK_DIR_NAME)?, false)? {
            if !is_dir(&internal) {
                continue;
            }

            for tfidf_file in read_dir(&internal, false)? {
                if extension(&tfidf_file).unwrap_or(None).unwrap_or(String::new()) == "tfidf" {
                    result.push(tfidf_file.to_string());
                }
            }
        }

        // the result has to be deterministic
        result.sort();
        Ok(result)
    }

    pub fn get_all_image_files(&self) -> Result<Vec<Path>, Error> {
        let mut result = vec![];

        for internal in read_dir(&join3(&self.root_dir, &INDEX_DIR_NAME, &IMAGE_DIR_NAME)?, false)? {
            if !is_dir(&internal) {
                continue;
            }

            for image_file in read_dir(&internal, false)? {
                if extension(&image_file).unwrap_or(None).unwrap_or(String::new()) == "png" {
                    result.push(image_file.to_string());
                }
            }
        }

        // the result has to be deterministic
        result.sort();
        Ok(result)
    }

    pub(crate) fn get_all_file_indexes(&self) -> Result<Vec<Path>, Error> {
        let mut result = vec![];

        for internal in read_dir(&join3(&self.root_dir, &INDEX_DIR_NAME, &FILE_INDEX_DIR_NAME)?, false)? {
            if !is_dir(&internal) {
                continue;
            }

            for file_index in read_dir(&internal, false)? {
                result.push(file_index.into());
            }
        }

        // the result has to be deterministic
        result.sort();
        Ok(result)
    }
}
