use crate::index_struct::Index;
use ragit_types::ApiError;
use std::path::PathBuf;
use ragit_types::uid::Uid;

pub fn index_get_uid_path(
    _index: &Index,
    _root_dir: &str,
    _dir_name: &str,
    _uid: Uid,
    _extension: Option<&str>,
) -> Result<PathBuf, ApiError> {
    Ok(PathBuf::new())
}
