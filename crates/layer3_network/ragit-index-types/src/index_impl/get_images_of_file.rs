use crate::index_struct::Index;
use ragit_types::ApiError;
use ragit_types::uid::Uid;

impl Index {
    pub fn get_images_of_file(&self, file_uid: Uid) -> Result<Vec<Uid>, ApiError> {
        eprintln!("Placeholder for get_images_of_file: file_uid={}", file_uid);
        Ok(vec![])
    }
}