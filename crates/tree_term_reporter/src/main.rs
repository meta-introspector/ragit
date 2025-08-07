use std::collections::HashMap;
use std::path::PathBuf;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct TermReport {
    terms: Vec<TermEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TermEntry {
    term: String,
    count: usize,
    is_internal: bool,
}

fn add_terms_from_string(s: &str, relative_path_str: &str, is_internal_path: bool, term_counts: &mut HashMap<String, (usize, bool)>, term_path_map: &mut HashMap<String, Vec<String>>) {
    let lower_s = s.to_lowercase();
    let entry = term_counts.entry(lower_s.clone()).or_insert((0, false));
    entry.0 += 1;
    entry.1 = entry.1 || is_internal_path; // If any path is internal, the term is internal
    term_path_map.entry(lower_s.clone()).or_insert_with(Vec::new).push(relative_path_str.to_string());

    for part in lower_s.split('_') {
        if !part.is_empty() && part != lower_s {
            let entry = term_counts.entry(part.to_string()).or_insert((0, false));
            entry.0 += 1;
            entry.1 = entry.1 || is_internal_path;
            term_path_map.entry(part.to_string()).or_insert_with(Vec::new).push(relative_path_str.to_string());
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_dir = PathBuf::from("/data/data/com.termux/files/home/storage/github/ragit");
    let output_dir = root_dir.join("index/solfunmeme-index");
    std::fs::create_dir_all(&output_dir)?;

    let mut term_counts: HashMap<String, (usize, bool)> = HashMap::new(); // (count, is_internal)
    let mut term_path_map: HashMap<String, Vec<String>> = HashMap::new();

    let ignored_patterns = vec![
        ".git",
        "target",
        "node_modules",
        "__pycache__",
        ".DS_Store",
        ".idea",
        ".vscode",
        ".ragit",
        "vendor",
        "index/solfunmeme-index",
        "logs",
        "tmp_bootstrap",
        "comms",
        "RelNotes",
    ];

    println!("Collecting terms from: {}", root_dir.display());
    for entry in WalkDir::new(&root_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path().to_path_buf();
        let relative_path = path.strip_prefix(&root_dir).unwrap_or(&path);
        let relative_path_str = relative_path.to_string_lossy().to_string();

        let mut should_ignore = false;
        for pattern in &ignored_patterns {
            if relative_path_str.contains(pattern) {
                should_ignore = true;
                break;
            }
        }

        if !should_ignore {
            let is_internal_path = relative_path_str.starts_with("crates/") &&
                                   !relative_path_str.contains("vendor/") &&
                                   !relative_path_str.contains("index/solfunmeme-index");

            // Extract terms from path components
            for component in path.components() {
                if let Some(s) = component.as_os_str().to_str() {
                    add_terms_from_string(s, &relative_path_str, is_internal_path, &mut term_counts, &mut term_path_map);
                }
            }

            // Extract terms from full file name
            if let Some(file_name_os) = path.file_name() {
                if let Some(s) = file_name_os.to_str() {
                    add_terms_from_string(s, &relative_path_str, is_internal_path, &mut term_counts, &mut term_path_map);
                    // Remove .md extension if present
                    if s.ends_with(".md") {
                        if let Some(stem) = s.strip_suffix(".md") {
                            add_terms_from_string(stem, &relative_path_str, is_internal_path, &mut term_counts, &mut term_path_map);
                        }
                    }
                }
            }

            // Extract terms from file extension
            if let Some(extension) = path.extension() {
                if let Some(s) = extension.to_str() {
                    add_terms_from_string(s, &relative_path_str, is_internal_path, &mut term_counts, &mut term_path_map);
                }
            }
        }
    }

    let mut internal_terms: Vec<TermEntry> = Vec::new();
    let mut external_terms: Vec<TermEntry> = Vec::new();
    let mut internal_term_path_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut external_term_path_map: HashMap<String, Vec<String>> = HashMap::new();

    for (term, (count, is_internal)) in term_counts {
        if is_internal {
            internal_terms.push(TermEntry { term: term.clone(), count, is_internal });
            if let Some(paths) = term_path_map.get(&term) {
                internal_term_path_map.insert(term, paths.clone());
            }
        } else {
            external_terms.push(TermEntry { term: term.clone(), count, is_internal });
            if let Some(paths) = term_path_map.get(&term) {
                external_term_path_map.insert(term, paths.clone());
            }
        }
    }

    internal_terms.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.term.cmp(&b.term)));
    external_terms.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.term.cmp(&b.term)));

    let internal_report = TermReport {
        terms: internal_terms,
    };
    let external_report = TermReport {
        terms: external_terms,
    };

    std::fs::write(output_dir.join("tree_term_report_internal.json"), serde_json::to_string_pretty(&internal_report)?.as_bytes())?;
    println!("Internal term report generated and saved to {}.", output_dir.join("tree_term_report_internal.json").display());
    std::fs::write(output_dir.join("tree_term_report_external.json"), serde_json::to_string_pretty(&external_report)?.as_bytes())?;
    println!("External term report generated and saved to {}.", output_dir.join("tree_term_report_external.json").display());

    std::fs::write(output_dir.join("term_path_map_internal.json"), serde_json::to_string_pretty(&internal_term_path_map)?.as_bytes())?;
    println!("Internal term path map generated and saved to {}.", output_dir.join("term_path_map_internal.json").display());
    std::fs::write(output_dir.join("term_path_map_external.json"), serde_json::to_string_pretty(&external_term_path_map)?.as_bytes())?;
    println!("External term path map generated and saved to {}.", output_dir.join("term_path_map_external.json").display());

    Ok(())
}