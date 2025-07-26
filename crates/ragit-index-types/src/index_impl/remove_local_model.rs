use crate::index_struct::Index;
use ragit_error::ApiError;

impl Index {
    pub fn remove_local_model(&mut self, model_name: &str) -> Result<(), ApiError> {
        eprintln!("Placeholder for remove_local_model: model_name={}", model_name);
        Ok(())
    }
}