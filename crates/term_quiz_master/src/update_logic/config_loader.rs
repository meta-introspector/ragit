use std::error::Error;
use std::path::PathBuf;
use std::fs;
use std::collections::HashSet;
use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
struct StopwordsConfig {
    stopwords: Vec<String>,
}

pub fn load_stopwords(repo_path: &PathBuf) -> Result<HashSet<String>, Box<dyn Error>> {
    let stopwords_path = repo_path.join("stopwords.toml");
    let stopwords_content = fs::read_to_string(&stopwords_path)?;
    let stopwords_config: StopwordsConfig = toml::from_str(&stopwords_content)?;
    Ok(stopwords_config.stopwords.into_iter().collect())
}
