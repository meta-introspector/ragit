use crate::index_struct::Index;
use ragit_error::ApiError;

impl Index {
    pub fn version(&self) -> Result<(), ApiError> {
        eprintln!("Placeholder for version");
        Ok(())
    }
}