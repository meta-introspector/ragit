use crate::index_struct::Index;
use ragit_types::ApiError;

impl Index {
    pub fn version(&self) -> Result<(), ApiError> {
        eprintln!("Placeholder for version");
        Ok(())
    }
}