use crate::prelude::*;
use ragit_fs::{extension, is_dir, join3, read_dir};

use std::path::PathBuf;
use crate::index::index_struct::Index;

impl Index {
    pub fn get_all_chunk_files(&self) -> Result<Vec<PathBuf>> {
        let mut result = vec![];

        for internal in read_dir(&join3(&self.root_dir, &INDEX_DIR_NAME.to_string(), &CHUNK_DIR_NAME.to_string())?, false)? {
            if !is_dir(&internal) {
                continue;
            }

            for chunk_file in read_dir(&internal, false)? {
                if extension(&chunk_file).unwrap_or(None).unwrap_or(String::new()) == "chunk" {
                    result.push(chunk_file.into());
                }
            }
        }

        // the result has to be deterministic
        result.sort();
        Ok(result)
    }

    pub fn get_all_tfidf_files(&self) -> Result<Vec<PathBuf>> {
        let mut result = vec![];

        for internal in read_dir(&join3(&self.root_dir, &INDEX_DIR_NAME.to_string(), &CHUNK_DIR_NAME.to_string())?, false)? {
            if !is_dir(&internal) {
                continue;
            }

            for tfidf_file in read_dir(&internal, false)? {
                if extension(&tfidf_file).unwrap_or(None).unwrap_or(String::new()) == "tfidf" {
                    result.push(tfidf_file.into());
                }
            }
        }

        // the result has to be deterministic
        result.sort();
        Ok(result)
    }

    pub fn get_all_image_files(&self) -> Result<Vec<PathBuf>> {
        let mut result = vec![];

        for internal in read_dir(&join3(&self.root_dir, &INDEX_DIR_NAME.to_string(), &IMAGE_DIR_NAME.to_string())?, false)? {
            if !is_dir(&internal) {
                continue;
            }

            for image_file in read_dir(&internal, false)? {
                if extension(&image_file).unwrap_or(None).unwrap_or(String::new()) == "png" {
                    result.push(image_file.into());
                }
            }
        }

        // the result has to be deterministic
        result.sort();
        Ok(result)
    }

    pub(crate) fn get_all_file_indexes(&self) -> Result<Vec<PathBuf>> {
        let mut result = vec![];

        for internal in read_dir(&join3(&self.root_dir, &INDEX_DIR_NAME.to_string(), &FILE_INDEX_DIR_NAME.to_string())?, false)? {
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
