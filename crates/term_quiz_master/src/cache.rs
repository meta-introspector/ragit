use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitCountCache {
    pub head_hash: String,
    pub total_commits: usize,
}

impl CommitCountCache {
    fn cache_file_path() -> PathBuf {
        PathBuf::from("/data/data/com.termux/files/home/storage/github/ragit/.ragit/term_quiz_master_cache.toml")
    }

    pub fn load() -> Result<Option<Self>, Box<dyn Error>> {
        let path = Self::cache_file_path();
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let cache: Self = toml::from_str(&content)?;
            Ok(Some(cache))
        } else {
            Ok(None)
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let path = Self::cache_file_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(&path, content.as_bytes())?;
        Ok(())
    }
}
