use crate::index_struct::Index;
use ragit_error::ApiError;

impl Index {
    pub async fn qa_test(&mut self) -> Result<(), ApiError> {
        eprintln!("Placeholder for qa_test");
        Ok(())
    }
}