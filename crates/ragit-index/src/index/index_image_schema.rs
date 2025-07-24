use crate::prelude::*;

impl Index {
    pub fn get_image_schema(&self, uid: Uid, load_bytes: bool) -> Result<ImageSchema, ApiError> {
        let image_schema_path = get_uid_path(
            &self.root_dir,
            &Path::new(IMAGE_DIR_NAME),
            uid,
            None,
        )?;

        if !exists(&image_schema_path) {
            return Err(ApiError::NoSuchFile {
                file: image_schema_path.to_string_lossy().to_string(),
                similar_files: vec![],
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