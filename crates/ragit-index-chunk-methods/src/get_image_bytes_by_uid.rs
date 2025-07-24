use crate::prelude::*;
use std::path::Path;

pub fn get_image_bytes_by_uid(index: &Index, uid: Uid) -> Result<Vec<u8>, ApiError> {
    Ok(read_bytes(&get_uid_path(&index.root_dir, Path::new(IMAGE_DIR_NAME), uid, Some("png"))?.to_string_lossy().into_owned())?)
}
