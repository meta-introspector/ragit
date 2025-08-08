use anyhow::Result;
use std::path::PathBuf;
use text_splitter::{TextSplitter, Characters};
use std::fs;
use ragit_types::build_config::BuildConfig;
use ragit_index_types::index_struct::Index;
use ragit_memory_monitor::MemoryMonitor;
use super::add_chunk_to_index::add_chunk_to_index;
use std::collections::HashSet;

pub fn process_staged_file(
    file_path_buf: &PathBuf,
    splitter: &TextSplitter<Characters>,
    build_config: &BuildConfig,
    index: &mut Index,
    memory_monitor: &mut MemoryMonitor,
    seen_keywords: &mut HashSet<String>,
) -> Result<()> {
    let content = fs::read_to_string(file_path_buf)?;
    let chunks: Vec<&str> = splitter.chunks(&content, build_config.chunk_size).collect();

    let mut file_keywords: Vec<String> = content
        .split_whitespace()
        .filter_map(|word| {
            let cleaned_word = word.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric()).to_string();
            if cleaned_word.is_empty() { None } else { Some(cleaned_word) }
        })
        .collect();
    file_keywords.dedup(); // Remove duplicates within the file's keywords
    println!("DEBUG: File: {}, Extracted keywords: {:?}", file_path_buf.file_name().unwrap().to_string_lossy(), file_keywords);

    let mut keywords_for_log: Vec<String> = Vec::new();
    let mut new_keywords_found = false;

    for keyword in &file_keywords {
        if !seen_keywords.contains(keyword) {
            keywords_for_log.push(keyword.clone());
            seen_keywords.insert(keyword.clone());
            new_keywords_found = true;
        }
    }
    println!("DEBUG: File: {}, Seen keywords after processing: {:?}", file_path_buf.file_name().unwrap().to_string_lossy(), seen_keywords);

    if !new_keywords_found && !file_keywords.is_empty() {
        // If no new keywords, take a few existing ones for the log
        keywords_for_log.extend(file_keywords.iter().take(3).cloned());
    }
    println!("DEBUG: File: {}, Keywords for log: {:?}", file_path_buf.file_name().unwrap().to_string_lossy(), keywords_for_log);

    let keywords_str = if keywords_for_log.is_empty() {
        String::from("")
    } else {
        format!(" [{}]", keywords_for_log.join(", "))
    };

    memory_monitor.verbose(&format!("{}{}", file_path_buf.file_name().unwrap().to_string_lossy(), keywords_str));
    let file_path_str = file_path_buf.to_string_lossy().to_string();
    let mut chunk_index = 0;
    for chunk_data in chunks {
        add_chunk_to_index(index, memory_monitor, chunk_data, &file_path_str, chunk_index);
        chunk_index += 1;
    }
    Ok(())
}
