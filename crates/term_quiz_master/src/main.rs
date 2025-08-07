use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json;
use rand::seq::SliceRandom;
use clap::Parser;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TermEntry {
    term: String,
    count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct TermReport {
    terms: Vec<TermEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TermPathMap {
    #[serde(flatten)]
    map: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AugmentedTermEntry {
    term: String,
    count: usize,
    category: String,
    significance: String,
    vibe: String,
    action_suggestion: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AugmentedTerms {
    augmented_terms: Vec<AugmentedTermEntry>,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The term to classify. If not provided, a random unclassified term will be displayed.
    #[clap(short, long)]
    term: Option<String>,

    /// The category for the term.
    #[clap(short, long)]
    category: Option<String>,

    /// The significance of the term.
    #[clap(short, long)]
    significance: Option<String>,

    /// The vibe of the term.
    #[clap(short, long)]
    vibe: Option<String>,

    /// An action suggestion for the term.
    #[clap(short, long)]
    action_suggestion: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let base_dir = PathBuf::from("/data/data/com.termux/files/home/storage/github/ragit/index/solfunmeme-index");
    let tree_term_report_path = base_dir.join("tree_term_report_internal.json");
    let augmented_terms_path = base_dir.join("augmented_terms_hot_take.json");
    let term_path_map_path = base_dir.join("term_path_map_internal.json");

    // Read tree_term_report.json
    let tree_term_report_content = fs::read_to_string(&tree_term_report_path)?;
    let tree_term_report: TermReport = serde_json::from_str(&tree_term_report_content)?;
    let all_terms: HashSet<String> = tree_term_report.terms.iter().map(|t| t.term.clone()).collect();

    // Read term_path_map.json
    let term_path_map_content = fs::read_to_string(&term_path_map_path)?;
    let term_path_map: TermPathMap = serde_json::from_str(&term_path_map_content)?;

    // Read augmented_terms_hot_take.json
    let mut augmented_terms: AugmentedTerms = if augmented_terms_path.exists() {
        let content = fs::read_to_string(&augmented_terms_path)?;
        serde_json::from_str(&content)?
    } else {
        AugmentedTerms { augmented_terms: Vec::new() }
    };

    let classified_terms: HashSet<String> = augmented_terms.augmented_terms.iter().map(|t| t.term.clone()).collect();

    // Identify unclassified terms
    let mut unclassified_terms: Vec<String> = all_terms
        .difference(&classified_terms)
        .cloned()
        .collect();
    unclassified_terms.shuffle(&mut rand::thread_rng());

    let mut new_classifications: Vec<AugmentedTermEntry> = Vec::new();

    let target_term_str = if let Some(term_arg) = args.term {
        if !all_terms.contains(&term_arg) {
            eprintln!("Error: Term '{}' not found in {}.", term_arg, tree_term_report_path.display());
            return Ok(());
        }
        if classified_terms.contains(&term_arg) {
            eprintln!("Error: Term '{}' is already classified.", term_arg);
            return Ok(());
        }
        term_arg
    } else {
        if unclassified_terms.is_empty() {
            println!("No unclassified terms remaining. Quiz complete!");
            return Ok(());
        }
        let term = unclassified_terms.first().unwrap().clone();
        println!("\n--- Term Quiz Master ---");
        println!("Found {} unclassified terms. Displaying one for classification:", unclassified_terms.len());
        println!("\nTerm: {}", term);

        // Display count
        let original_term_count = tree_term_report.terms.iter()
            .find(|t| t.term == term)
            .map_or(0, |t| t.count);
        println!("  Count: {}", original_term_count);

        // Display example paths
        if let Some(paths) = term_path_map.map.get(&term) {
            println!("  Example Paths (up to 3):");
            for (i, path) in paths.iter().take(3).enumerate() {
                println!("    {}. {}\n", i + 1, path);
            }
        }
        println!("\nTo classify this term, run with: --term \"{}\" --category \"...\" --significance \"...\" --vibe \"...\" --action-suggestion \"...\"", term);
        return Ok(());
    };

    // If we reach here, a term was provided via CLI and is unclassified
    let category = args.category.ok_or("Category is required when classifying a term.")?;
    let significance = args.significance.ok_or("Significance is required when classifying a term.")?;
    let vibe = args.vibe.ok_or("Vibe is required when classifying a term.")?;
    let action_suggestion = args.action_suggestion.ok_or("Action suggestion is required when classifying a term.")?;

    let original_term_count = tree_term_report.terms.iter()
        .find(|t| t.term == target_term_str)
        .map_or(0, |t| t.count);

    new_classifications.push(AugmentedTermEntry {
        term: target_term_str.clone(),
        count: original_term_count,
        category: category.trim().to_string(),
        significance: significance.trim().to_string(),
        vibe: vibe.trim().to_string(),
        action_suggestion: action_suggestion.trim().to_string(),
    });

    augmented_terms.augmented_terms.extend(new_classifications);

    // Write updated augmented_terms_hot_take.json
    let updated_json = serde_json::to_string_pretty(&augmented_terms)?;
    fs::write(&augmented_terms_path, updated_json.as_bytes())?;

    println!("\nSuccessfully classified '{}'. Updated {}.", target_term_str, augmented_terms_path.display());

    Ok(())
}