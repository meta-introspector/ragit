use crate::prelude::*;
use ragit_index_types::Index;

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
            result.bytes = Some(read_bytes(&result.image_path)?);
        }

        Ok(result)
    }
}