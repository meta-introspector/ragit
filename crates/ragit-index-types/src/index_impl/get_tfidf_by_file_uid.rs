use crate::index_struct::Index;
use ragit_error::ApiError;
use ragit_types::uid::Uid;
use ragit_types::processed_doc::ProcessedDoc;

impl Index {
    pub fn get_tfidf_by_file_uid(
        &self,
        file_uid: Uid,
    ) -> Result<ProcessedDoc, ApiError> {
        eprintln!(
            "Placeholder for get_tfidf_by_file_uid: file_uid={}",
            file_uid
        );
        Err(ApiError::BrokenIndex("Placeholder for get_tfidf_by_file_uid".to_string()))
    }
}