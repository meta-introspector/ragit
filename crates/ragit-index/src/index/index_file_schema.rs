use crate::prelude::*;

impl Index {
    pub fn get_file_schema(&self, uid: Uid) -> Result<FileSchema, ApiError> {
        let file_schema_path = get_uid_path(
            &self.root_dir,
            &Path::new(FILE_INDEX_DIR_NAME),
            uid,
            None,
        )?;

        if !exists(&file_schema_path) {
            return Err(ApiError::NoSuchFile {
                file: file_schema_path.to_string_lossy().to_string(),
                similar_files: vec![],
            });
        }

        let s = read_string(&file_schema_path)?;
        Ok(serde_json::from_str(&s)?)
    }

    pub fn get_file_schema_from_path(
        &self,
        file_path: &PathBuf,
    ) -> Result<FileSchema, ApiError> {
        let uid = self
            .processed_files
            .get(file_path)
            .ok_or_else(|| ApiError::NoSuchFile { file: file_path.to_string_lossy().to_string(), similar_files: vec![] })?;

        self.get_file_schema(*uid)
    }

    pub fn get_chunk_build_info(
        &self,
        file_path: &PathBuf,
    ) -> Result<(Model, usize), ApiError> {
        let file_schema = self.get_file_schema_from_path(file_path)?;

        Ok(match file_schema.chunk_build_info {
            Some(chunk_build_info) => (
                self.models
                    .iter()
                    .find(|m| m.name == chunk_build_info.model)
                    .ok_or_else(|| ApiError::InvalidTestModel(chunk_build_info.model.clone()))?
                    .clone(),
                chunk_build_info.chunk_size,
            ),
            None => (ChunkBuildInfo::default().model, 0),
        })
    }
}