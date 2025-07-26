use crate::index_struct::Index;
use ragit_error::ApiError;
use std::collections::HashMap;

impl Index {
    pub fn get_all_meta(&self) -> Result<HashMap<String, String>, ApiError> {
        let result = HashMap::new();

        Ok(result)
    }
}