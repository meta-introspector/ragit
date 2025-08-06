use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use regex::Regex;
use walkdir::WalkDir;
use std::collections::HashMap;
use ragit_utils::memory_utils;
use sysinfo::System;

const CONTEXT_LINES: usize = 2; // Number of lines before and after the match

fn main() -> io::Result<()> {
    let mut sys = System::new_all();
    let mut last_snapshot_data = None;
    memory_utils::print_memory_usage(&mut sys, "Start of Indexing", &mut last_snapshot_data);

    let project_root = PathBuf::from("/data/data/com.termux/files/home/storage/github/ragit");
    let output_base_dir = project_root.join("index").join("terms");

    // Define interesting terms and their common misspellings/variations
    let interesting_terms = vec![
        ("goedel", vec!["goedel", "Gödel", "Godel"]),
        ("monster_group", vec!["monster group", "fischer-griess monster"]),
        ("oeis", vec!["oeis", "online encyclopedia of integer sequences"]),
        ("alife", vec!["alife", "artificial life"]),
        ("incompleteness", vec!["incompleteness", "incompleteness theorem", "incompleteness theroem", "incompleteness theory"]),
        ("formal_system", vec!["formal system", "formal systems"]),
        ("prime", vec!["prime", "prime number", "primes"]),
        ("bit_size", vec!["bit size", "bit-size"]),
        ("data_structure", vec!["data structure", "data structures"]),
        ("proof", vec!["proof", "proofs"]),
        ("axiom", vec!["axiom", "axioms"]),
        ("ontology", vec!["ontology", "ontologies"]),
        ("meme", vec!["meme", "memes"]),
        ("vibe", vec!["vibe", "vibes"]),
        ("solana", vec!["solana"]),
        ("zkp", vec!["zkp", "zero-knowledge proof", "zero knowledge proof", "zkps"]),
        ("blockchain", vec!["blockchain", "blockchains"]),
        ("metacoq", vec!["metacoq"]),
        ("turing", vec!["turing", "turing machine", "turing machines"]),
    ];

    let mut term_regexes: HashMap<String, Regex> = HashMap::new();
    for (base_term, variations) in &interesting_terms {
        let pattern_str = variations.iter()
            .map(|s| regex::escape(s))
            .collect::<Vec<String>>()
            .join("|");
        term_regexes.insert(base_term.to_string(), Regex::new(&format!("(?i){}", pattern_str)).unwrap());
    }

    for entry in WalkDir::new(&project_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == "md"))
    {
        let file_path = entry.path();
        let relative_path = file_path.strip_prefix(&project_root).unwrap_or(file_path).display().to_string();
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();

        for (line_num, line) in lines.iter().enumerate() {
            for (base_term, regex) in &term_regexes {
                if regex.is_match(line) {
                    let term_output_dir = output_base_dir.join(base_term);
                    fs::create_dir_all(&term_output_dir)?;

                    let output_file_name = format!("{}_{:04}.md", file_path.file_stem().unwrap_or_default().to_string_lossy(), line_num + 1);
                    let output_file_path = term_output_dir.join(output_file_name);
                    let mut output_file = File::create(&output_file_path)?;

                    // Write context before the match
                    for i in (line_num as isize - CONTEXT_LINES as isize)..line_num as isize {
                        if i >= 0 {
                            writeln!(output_file, "{}", lines[i as usize])?;
                        }
                    }

                    // Write the matching line
                    writeln!(output_file, "{}", line)?;

                    // Write context after the match
                    for i in (line_num + 1)..(line_num + 1 + CONTEXT_LINES) {
                        if i < lines.len() {
                            writeln!(output_file, "{}", lines[i])?;
                        }
                    }
                    writeln!(output_file, "\n--- Source: {} (Line {}) ---", relative_path, line_num + 1)?;
                }
            }
        }
    }

    memory_utils::print_memory_usage(&mut sys, "End of Indexing", &mut last_snapshot_data);
    println!("Eco-style indexing complete. Output written to {:?}", output_base_dir);
    Ok(())
}