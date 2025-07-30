use crate::index_struct::Index;
use ragit_types::ApiError;
use ragit_types::FileSchema;
use ragit_types::uid::Uid;
use std::path::PathBuf;

impl Index {
    pub fn get_file_schema(
        &self,
        path: Option<PathBuf>,
        uid: Option<Uid>,
    ) -> Result<FileSchema, ApiError> {
        eprintln!(
            "Placeholder for get_file_schema: path={:?}, uid={:?}",
            path, uid
        );
        Err(ApiError::BrokenIndex("Placeholder for get_file_schema".to_string()))
    }
}