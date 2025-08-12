use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;
use std::path::PathBuf;
use walkdir::WalkDir;
use serde_json;


fn get_memory_usage() -> Option<usize> {
    let statm_path = "/proc/self/statm";
    let content = std::fs::read_to_string(statm_path).ok()?;
    let parts: Vec<&str> = content.split_whitespace().collect();
    if parts.len() > 1 {
        // Resident Set Size (RSS) is the second field, in pages. Convert to KB.
        parts[1].parse::<usize>().ok().map(|pages| pages * 4) // Assuming 4KB page size
    } else {
        None
    }
}

fn main() -> io::Result<()> {
    println!("Starting term report generation...");
    let terms_dir = PathBuf::from("/data/data/com.termux/files/home/storage/github/ragit/index/solfunmeme-index/terms/");
    let word_counter_path = PathBuf::from("/data/data/com.termux/files/home/storage/github/ragit/target/release/word_counter");

    let mut term_info: Vec<(PathBuf, u64)> = Vec::new();

    println!("Reading term directories from: {}", terms_dir.display());
    for entry in std::fs::read_dir(&terms_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let mut total_size = 0;
            println!("  Calculating size for term directory: {}", path.display());
            for file_entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                if file_entry.file_type().is_file() && file_entry.path().extension().map_or(false, |ext| ext == "md") {
                    if let Ok(metadata) = file_entry.metadata() {
                        total_size += metadata.len();
                    }
                }
            }
            term_info.push((path, total_size));
        }
    }

    println!("Sorting terms by size...");
    term_info.sort_by_key(|k| k.1);

    for (term_path, _size) in term_info {
        let term_name = term_path.file_name().unwrap().to_string_lossy().to_string();
        println!("
Processing term: {}", term_name);

        let mut report_content = String::new();
        report_content.push_str(&format!("# Term: {}

", term_name));

        let mut files_processed_count = 0;
        println!("  Counting index cards (md files) in {}...", term_path.display());
        for file_entry in WalkDir::new(&term_path).into_iter().filter_map(|e| e.ok()) {
            if file_entry.file_type().is_file() && file_entry.path().extension().map_or(false, |ext| ext == "md") {
                files_processed_count += 1;
            }
        }

        println!("  Running word_counter for {} files...", files_processed_count);
        let output = Command::new(&word_counter_path)
            .arg(term_path.to_string_lossy().to_string())
            .output()?;

        if output.status.success() {
            println!("  Word counter executed successfully.");
            let stdout = String::from_utf8_lossy(&output.stdout);
            let word_counts: HashMap<String, usize> = serde_json::from_str(&stdout)?;

            let mut sorted_words: Vec<(&String, &usize)> = word_counts.iter().collect();
            sorted_words.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

            report_content.push_str("| Word | Count |
");
            report_content.push_str("|------|-------|
");
            for (word, count) in sorted_words.iter() { // Display all words
                report_content.push_str(&format!("| {} | {} |
", word, count));
            }
            report_content.push_str("
");
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            report_content.push_str(&format!("Error processing term {}: {}

", term_name, stderr));
            eprintln!("Error from word_counter for {}: {}
", term_name, stderr);
        }

        let output_path = term_path.join(format!("{}_report.md", term_name));
        println!("  Creating report file: {}", output_path.display());
        let mut file = std::fs::File::create(&output_path)?;
        file.write_all(report_content.as_bytes())?;

        println!("  Term report generated at {}", output_path.display());
        if let Some(mem_kb) = get_memory_usage() {
            println!("  Memory usage: {} KB", mem_kb);
        }
        println!("  Index cards processed for {}: {}", term_name, files_processed_count);

    }

    println!("Term report generation completed.");

    Ok(())
}
