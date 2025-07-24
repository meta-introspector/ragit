use super::prelude::*;
use crate::index_struct::Index;
use ragit_utils::ragit_path_utils::get_uid_path;
use ragit_fs::exists;
use ragit_error::ApiError;
use ragit_types::Uid;
use ragit_tfidf::load_from_file as load_processed_doc_from_file;

impl Index {
    pub fn calculate_uid(&self, force: bool) -> Result<Uid, ApiError> {
        if !force && self.uid != Uid::dummy() {
            return Ok(self.uid);
        }

        let mut uids = vec![];

        for file in self.processed_files.keys() {
            let file_index_path = get_uid_path(&self.root_dir, FILE_INDEX_DIR_NAME, Uid::dummy(), Some(file.to_str().unwrap()))?;

            if exists(&file_index_path) {
                let processed_doc = load_processed_doc_from_file(file_index_path.to_str().unwrap())?;
                uids.push(processed_doc.uid);
            }
        }

        uids.sort();

        Ok(if uids.is_empty() { Uid::dummy() } else { uids[0] })
    }
}
