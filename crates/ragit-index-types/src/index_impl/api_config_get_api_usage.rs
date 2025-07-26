use crate::index_struct::Index;
use ragit_error::ApiError;
use std::collections::HashMap;
use std::path::PathBuf;
use ragit_api::AuditRecord;

pub fn index_api_config_get_api_usage(
    index: &Index,
    root_dir: &PathBuf,
    name: &str,
) -> Result<HashMap<String, AuditRecord>, ApiError> {
    index.api_config.get_api_usage(root_dir, name)
}
