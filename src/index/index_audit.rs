use crate::prelude::*;

impl Index {
    pub fn audit(&self, since: Option<chrono::DateTime<chrono::Local>>) -> Result<HashMap<String, Audit>> {
        let mut result = HashMap::new();
        let audit_path = self.get_rag_path().join("audit");
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
                if chrono::NaiveDateTime::from_timestamp(timestamp, 0) < since.naive_local() {
                    continue;
                }
            }
            let audit: Audit = serde_json::from_str(&std::fs::read_to_string(&path)?)?;
            *result.entry(category).or_default() += audit;
        }
        Ok(result)
    }
}
