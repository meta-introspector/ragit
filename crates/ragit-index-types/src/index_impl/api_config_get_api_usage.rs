use crate::index_struct::Index;
use ragit_types::ApiError;
use std::collections::HashMap;
use std::path::PathBuf;
use ragit_api::AuditRecord;

pub fn index_api_config_get_api_usage(
    _index: &Index,
    _root_dir: &PathBuf,
    _name: &str,
) -> Result<HashMap<String, AuditRecord>, ApiError> {
    Ok(HashMap::new())
}
