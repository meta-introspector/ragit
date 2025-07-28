use crate::index_struct::Index;
use ragit_types::ApiError;

impl Index {
    pub fn status(&self) -> Result<(), ApiError> {
        eprintln!("Placeholder for status");
        Ok(())
    }
}