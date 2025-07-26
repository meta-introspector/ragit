use ragit_index_types::index_struct::Index;
use ragit_error::ApiError;
use ragit_api::ModelRaw;

impl Index {
    pub fn get_initial_models() -> Result<Vec<ModelRaw>, ApiError> {
        eprintln!("Placeholder for get_initial_models");
        Ok(vec![])
    }
}