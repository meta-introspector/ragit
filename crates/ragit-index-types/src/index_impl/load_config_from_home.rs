use crate::index_struct::Index;
use ragit_error::ApiError;
use serde::de::DeserializeOwned;

impl Index {
    pub fn load_config_from_home<T: DeserializeOwned>(
        file_name: &str,
    ) -> Result<Option<T>, ApiError> {
        eprintln!("Placeholder for load_config_from_home: file_name={}", file_name);
        Ok(None)
    }
}