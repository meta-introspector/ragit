use std::error::Error;
use std::path::PathBuf;
use std::fs;
use serde_json;
use std::io::Write;
use crate::augmented_term_entry::AugmentedTermEntry;
use crate::augmented_terms::AugmentedTerms;
use std::collections::HashMap;

pub fn finalize_index(
    terms_map: HashMap<String, AugmentedTermEntry>,
    base_dir: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    let augmented_terms_path = base_dir.join("augmented_terms_hot_take.0.json");
    let jsonl_path = base_dir.join("augmented_terms.jsonl");

    let augmented_terms = AugmentedTerms { augmented_terms: terms_map.into_values().collect() };

    // Calculate total version entries for memory usage proxy
    let total_version_entries: usize = augmented_terms.augmented_terms.iter()
        .map(|entry| entry.versions.len())
        .sum();

    println!("\n--- Finalizing Index ---");
    println!("Total unique terms collected: {}", augmented_terms.augmented_terms.len());
    println!("Total term-version entries: {}", total_version_entries);

    // Write to augmented_terms_hot_take.json (pretty-printed)
    let updated_json = serde_json::to_string_pretty(&augmented_terms)?;
    fs::write(&augmented_terms_path, updated_json.as_bytes())?;
    println!("Updated augmented_terms_hot_take.json with terms from Git history.");

    // Write to JSON Lines format for Hugging Face compatibility
    let mut jsonl_file = fs::File::create(&jsonl_path)?;
    for entry in &augmented_terms.augmented_terms {
        let json_line = serde_json::to_string(&entry)?;
        writeln!(jsonl_file, "{}", json_line)?;
    }
    println!("Saved augmented terms to Hugging Face compatible JSONL: {}", jsonl_path.display());

    Ok(())
}
