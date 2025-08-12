use std::collections::HashSet;
use std::path::PathBuf;
use std::fs;
use serde_json;
use rand::seq::SliceRandom;
use clap::Parser;

use super::cli::Args;
use super::term_report::TermReport;
use super::term_path_map::TermPathMap;
use super::augmented_term_entry::AugmentedTermEntry;
use super::augmented_terms::AugmentedTerms;
use crate::update_logic::run_update_command;
// use hf_dataset_validator::hf_dataset_converter;
// use tokio;

pub fn run_quiz() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.update {
        run_update_command(&args)?;
        return Ok(());
    }

    let base_dir = PathBuf::from("./idx");
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

    // if args.generate_parquet {
    //     println!("Generating Parquet files...");
    //     let output_dir = PathBuf::from("./idx"); // Output to the idx submodule
    //     hf_dataset_validator::hf_dataset_converter::create_huggingface_dataset_from_augmented_terms(&augmented_terms, &output_dir.to_string_lossy()).await?;
    //     println!("Parquet files generated successfully in {}.", output_dir.display());
    //     return Ok(()); // Exit after generating parquet
    // }

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
        emoji_representation: None,
        semantic_names: None,
        osi_layer: None,
        prime_factor: None,
        is_power_of_two: None,
        numerical_address: None,
        embedding_vectors: None,
        versions: Vec::new(),
        first_seen_timestamp: None,
        last_seen_timestamp: None,
    });

    augmented_terms.augmented_terms.extend(new_classifications);

    // Write updated augmented_terms_hot_take.json
    let updated_json = serde_json::to_string_pretty(&augmented_terms)?;
    fs::write(&augmented_terms_path, updated_json.as_bytes())?;

    println!("\nSuccessfully classified '{}'. Updated {}.", target_term_str, augmented_terms_path.display());

    if args.generate_parquet {
        println!("Generating Parquet files...");
        let output_dir = PathBuf::from("./idx"); // Output to the idx submodule
        hf_dataset_converter::create_huggingface_dataset(&augmented_terms_path.to_string_lossy(), &output_dir.to_string_lossy()).await?;
        println!("Parquet files generated successfully in {}.", output_dir.display());
    }

    Ok(())
}