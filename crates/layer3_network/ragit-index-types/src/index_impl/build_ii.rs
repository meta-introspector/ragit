use crate::index_struct::Index;
use ragit_types::ApiError;

impl Index {
    pub fn build_ii(&mut self, quiet: bool) -> Result<(), ApiError> {
        eprintln!("Placeholder for build_ii: quiet={}", quiet);
        Ok(())
    }
}