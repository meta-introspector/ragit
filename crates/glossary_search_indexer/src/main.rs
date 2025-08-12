use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use regex::Regex;
use walkdir::WalkDir;

const CHUNK_SIZE: usize = 1000; // Lines per chunk for output

fn main() -> io::Result<()> {
    let glossary_dir = PathBuf::from("/data/data/com.termux/files/home/storage/github/ragit/docs/glossary");
    let project_root = PathBuf::from("/data/data/com.termux/files/home/storage/github/ragit");
    let output_dir = project_root.join("index").join("terms").join("goedel");

    fs::create_dir_all(&output_dir)?;

    let glossary_terms = get_glossary_terms(&glossary_dir)?;
    let search_patterns = generate_search_patterns(&glossary_terms);

    let mut current_chunk_lines: Vec<String> = Vec::new();
    let mut chunk_number = 0;
    let mut line_count = 0;

    for entry in WalkDir::new(&project_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == "md"))
    {
        let file_path = entry.path();
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        for (line_num, line) in reader.lines().enumerate() {
            let line = line?;
            for (pattern, term) in &search_patterns {
                if pattern.is_match(&line) {
                    let relative_path = file_path.strip_prefix(&project_root).unwrap_or(file_path).display().to_string();
                    current_chunk_lines.push(format!("File: {} | Line: {} | Term: {} | Content: {}", relative_path, line_num + 1, term, line));
                    line_count += 1;

                    if line_count % CHUNK_SIZE == 0 {
                        write_chunk(&output_dir, chunk_number, &current_chunk_lines)?;
                        current_chunk_lines.clear();
                        chunk_number += 1;
                    }
                    break; // Move to next line after first match
                }
            }
        }
    }

    // Write any remaining lines in the last chunk
    if !current_chunk_lines.is_empty() {
        write_chunk(&output_dir, chunk_number, &current_chunk_lines)?;
    }

    println!("Indexing complete. Output written to {:?}", output_dir);
    Ok(())
}

fn get_glossary_terms(glossary_dir: &Path) -> io::Result<Vec<String>> {
    let mut terms = Vec::new();
    for entry in fs::read_dir(glossary_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            if let Some(file_stem) = entry.path().file_stem() {
                terms.push(file_stem.to_string_lossy().into_owned());
            }
        }
    }
    Ok(terms)
}

fn generate_search_patterns(terms: &[String]) -> Vec<(Regex, String)> {
    let mut patterns = Vec::new();
    for term in terms {
        // Generate patterns for exact match, and common misspellings/variations
        let pattern_str = format!("(?i){}|{}|{}",
                                  regex::escape(term),
                                  regex::escape(&term.replace("goedel", "godel").replace("Gödel", "Godel")),
                                  regex::escape(&term.replace("godel", "goedel").replace("Godel", "Gödel")));
        patterns.push((Regex::new(&pattern_str).unwrap(), term.clone()));
    }
    patterns
}

fn write_chunk(output_dir: &Path, chunk_number: usize, lines: &[String]) -> io::Result<()> {
    let output_file_path = output_dir.join(format!("chunk_{:04}.txt", chunk_number));
    let mut file = File::create(&output_file_path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}