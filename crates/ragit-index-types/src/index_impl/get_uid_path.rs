use crate::index_struct::Index;
use ragit_error::ApiError;
use std::path::PathBuf;
use ragit_types::uid::Uid;

pub fn index_get_uid_path(
    index: &Index,
    root_dir: &str,
    dir_name: &str,
    uid: Uid,
    extension: Option<&str>,
) -> Result<PathBuf, ApiError> {
    index.get_uid_path(root_dir, dir_name, uid, extension)
}
