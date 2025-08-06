use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use regex::Regex;
use serde::Deserialize;

const CHUNK_SIZE: usize = 1000; // Lines per chunk

#[derive(Debug, Deserialize)]
struct StopwordsConfig {
    stopwords: Vec<String>,
}

fn main() -> io::Result<()> {
    let file_path = "/data/data/com.termux/files/home/storage/github/ragit/index/terms/goedel/goedel.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let stopwords_path = "/data/data/com.termux/files/home/storage/github/ragit/crates/word_counter/stopwords.toml";
    let stopwords_content = std::fs::read_to_string(stopwords_path)?;
    let stopwords_config: StopwordsConfig = toml::from_str(&stopwords_content)?;
    let stopwords: Vec<String> = stopwords_config.stopwords.into_iter().map(|s| s.to_lowercase()).collect();

    let mut chunk_number = 0;
    let mut current_chunk_lines = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        current_chunk_lines.push(line);

        if (index + 1) % CHUNK_SIZE == 0 {
            process_chunk(chunk_number, &current_chunk_lines, &stopwords)?;
            current_chunk_lines.clear();
            chunk_number += 1;
        }
    }

    // Process any remaining lines in the last chunk
    if !current_chunk_lines.is_empty() {
        process_chunk(chunk_number, &current_chunk_lines, &stopwords)?;
    }

    Ok(())
}

fn process_chunk(chunk_number: usize, lines: &[String], stopwords: &[String]) -> io::Result<()> {
    let mut word_counts: HashMap<String, usize> = HashMap::new();
    let word_regex = Regex::new(r"\b\w+\b").unwrap();

    for line in lines {
        for mat in word_regex.find_iter(line) {
            let word = mat.as_str().to_lowercase();
            if !stopwords.contains(&word) {
                *word_counts.entry(word).or_insert(0) += 1;
            }
        }
    }

    println!("--- Chunk {} ---", chunk_number);
    let mut sorted_words: Vec<(&String, &usize)> = word_counts.iter().collect();
    sorted_words.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

    for (word, count) in sorted_words.iter().take(20) { // Display top 20 words
        println!("{}: {}", word, count);
    }
    println!("Total unique words: {}", word_counts.len());
    println!("\n");

    Ok(())
}
