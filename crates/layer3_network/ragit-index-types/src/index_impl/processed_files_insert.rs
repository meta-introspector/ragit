use crate::index_struct::Index;
use std::path::PathBuf;
use ragit_types::uid::Uid;

pub fn index_processed_files_insert(
    index: &mut Index,
    file: PathBuf,
    file_uid: Uid,
) {
    index.processed_files.insert(file, file_uid);
}
