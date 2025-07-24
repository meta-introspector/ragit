use crate::prelude::*;
use crate::index_struct::Index;
use ragit_utils::ragit_path_utils::get_uid_path;
use ragit_fs::{exists, read_string, read_bytes};
use ragit_error::ApiError;
use ragit_types::{ImageSchema, Uid};

impl Index {
    pub fn get_image_schema(&self, uid: Uid, load_bytes: bool) -> Result<ImageSchema, ApiError> {
        let image_schema_path = get_uid_path(
            &self.root_dir,
            IMAGE_DIR_NAME,
            uid,
            None,
        )?;

        if !exists(&image_schema_path) {
            return Err(ApiError::FileNotFound {
                path: image_schema_path.to_string_lossy().to_string(),
                similar_paths: vec![],
            });
        }

        let s = read_string(&image_schema_path)?;
        let mut result: ImageSchema = serde_json::from_str(&s)?;

        if load_bytes {
            result.bytes = read_bytes(image_schema_path.to_str().unwrap())?;
        }

        Ok(result)
    }
}
