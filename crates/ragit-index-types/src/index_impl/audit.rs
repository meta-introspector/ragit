use crate::index_struct::Index;
use ragit_types::{ApiError, read_dir_to_api_error, read_to_string_to_api_error, map_serde_json_error, map_io_error};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use ragit_api::AuditRecord;
use std::path::PathBuf;

impl Index {
    pub fn audit(
        &self,
        since: Option<DateTime<chrono::Local>>,
    ) -> Result<HashMap<String, AuditRecord>, ApiError> {
        let mut result = HashMap::new();
        let audit_path =
            ragit_utils::ragit_path_utils::get_rag_path(&self.root_dir, &PathBuf::from("audit"))
                .map_err(|e| ApiError::Internal(format!("Failed to get rag path: {}", e)))?
                .join("audit");
        if !audit_path.exists() {
            return Ok(result);
        }
        for entry in read_dir_to_api_error(&audit_path)? {
            let entry = map_io_error(entry)?;
            let path = entry.path();
            if path.is_dir() {
                continue;
            }
            let file_name = path.file_name().unwrap().to_string_lossy();
            let parts: Vec<&str> = file_name.splitn(2, '_').collect();
            if parts.len() != 2 {
                continue;
            }
            let timestamp = parts[0].parse::<i64>()?;
            let category = parts[1].to_string();
            if let Some(since) = since {
                if DateTime::<Utc>::from_timestamp(timestamp, 0).unwrap() < since.to_utc() {
                    continue;
                }
            }
            let audit: AuditRecord = map_serde_json_error(serde_json::from_str(&read_to_string_to_api_error(&path)?))?;
            *result.entry(category).or_default() += audit;
        }
        Ok(result)
    }
}