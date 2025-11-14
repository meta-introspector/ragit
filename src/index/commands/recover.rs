use crate::constant::INDEX_FILE_NAME;
use crate::error::Error;
use crate::index::index_struct::Index;
use ragit_fs::{remove_file, set_extension};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use crate::chunk;
use crate::index::tfidf;

#[derive(Debug, Deserialize, Serialize)]
pub struct RecoverResult {
    pub success: usize,
    pub errors: usize,
}

impl Index {
    pub fn recover(&mut self) -> Result<RecoverResult, Error> {
        let mut success = 0;
        let mut errors = 0;
        let mut all_chunk_uids = HashSet::new();

        for chunk_file in &self.get_all_chunk_files()? {
            let tfidf_file = set_extension(&chunk_file, "tfidf")?;

            if chunk::load_from_file(&chunk_file).is_err() || tfidf::load_from_file(&tfidf_file).is_err() {
                remove_file(&chunk_file)?;
                remove_file(&tfidf_file)?;
                errors += 1;
                continue;
            }

            let chunk_ = chunk::load_from_file(&chunk_file)?;
            all_chunk_uids.insert(chunk_.uid);
            success += 1;
        }

        let mut processed_files_to_remove = vec![];

        for (file, file_uid) in self.processed_files.iter() {
            let mut remove_file_ = false;

            for chunk_uid in crate::uid::load_from_file(&Index::get_file_index_path(&self.root_dir, *file_uid)?)? {
                if !all_chunk_uids.contains(&chunk_uid) {
                    remove_file_ = true;
                    break;
                }
            }

            if remove_file_ {
                processed_files_to_remove.push(file.to_string_lossy().into_owned());
            }
        }

        for file in processed_files_to_remove.iter() {
            self.processed_files.remove(file);
        }

        self.save_to_file(self.root_dir.join(INDEX_FILE_NAME.to_string()))?;
        self.calculate_and_save_uid()?;

        Ok(RecoverResult {
            success,
            errors,
        })
    }
}
