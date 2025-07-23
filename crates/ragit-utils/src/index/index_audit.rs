use crate::index::index_struct::Index;
use crate::prelude::*;
use chrono::{DateTime, Utc};
use ragit_api::AuditRecord;
use std::collections::HashMap;
use std::path::PathBuf;

impl Index {
    pub fn audit(
        &self,
        since: Option<DateTime<chrono::Local>>,
    ) -> Result<HashMap<String, AuditRecord>> {
        let mut result = HashMap::new();
        let audit_path =
            crate::path_utils::get_rag_path(&self.root_dir, &PathBuf::from("audit"))?.join("audit");
        if !audit_path.exists() {
            return Ok(result);
        }
        for entry in std::fs::read_dir(audit_path)? {
            let entry = entry?;
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
            let audit: AuditRecord = serde_json::from_str(&std::fs::read_to_string(&path)?)?;
            *result.entry(category).or_default() += audit;
        }
        Ok(result)
    }
}
