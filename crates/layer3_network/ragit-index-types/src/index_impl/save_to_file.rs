use crate::index_struct::Index;
use ragit_types::ApiError;
use std::path::PathBuf;
//use ragit_utils::constant::INDEX_FILE_NAME;

pub fn index_save_to_file(
    _index: &Index,
    _path: PathBuf,
) -> Result<(), ApiError> {
    Ok(())
}
