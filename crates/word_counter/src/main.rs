use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use regex::Regex;
use serde::Deserialize;
use serde_json;
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
struct StopwordsConfig {
    stopwords: Vec<String>,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <directory_path>", args[0]);
        std::process::exit(1);
    }
    let directory_path = &args[1];

    let stopwords_path = "/data/data/com.termux/files/home/storage/github/ragit/crates/word_counter/stopwords.toml";
    let stopwords_content = std::fs::read_to_string(stopwords_path)?;
    let stopwords_config: StopwordsConfig = toml::from_str(&stopwords_content)?;
    let stopwords: Vec<String> = stopwords_config.stopwords.into_iter().map(|s| s.to_lowercase()).collect();

    let mut word_counts: HashMap<String, usize> = HashMap::new();
    let word_regex = Regex::new(r"\b\w+\b").unwrap();

    for entry in WalkDir::new(directory_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let path_str = path.to_string_lossy();

        // Exclude submodules
        if path_str.contains("index/solfunmeme-index") || path_str.contains("vendor/meta-introspector/solfunmeme-dioxus") {
            continue;
        }

        // Exclude problematic godel test data
        if path_str.contains("godel25_") && path_str.ends_with(".md") {
            continue;
        }

        if entry.file_type().is_file() && path.extension().map_or(false, |ext| ext == "md") {
            let file = File::open(entry.path())?;
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line = line?;
                for mat in word_regex.find_iter(&line) {
                    let word = mat.as_str().to_lowercase();
                    if !stopwords.contains(&word) {
                        *word_counts.entry(word).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    let json_output = serde_json::to_string_pretty(&word_counts)?;
    println!("{}", json_output);

    Ok(())
}
