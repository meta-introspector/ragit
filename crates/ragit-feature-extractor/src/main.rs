use ragit_feature_extractor::get_sampled_reports;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use regex::Regex;
use std::fs;
use std::io::{BufReader, BufRead};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TermEmbeddings(HashMap<String, Vec<f64>>);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let selected_reports = get_sampled_reports()?;

    // Get top 10 global pairs and triples (re-calculate for printing)
    // This part is duplicated from get_sampled_reports for standalone binary execution
    // In a real application, global counts would be passed or stored differently.
    let term_embeddings_path = "term_embeddings.json";
    let index_file_path = "index.rs.txt";
    let word_regex = Regex::new(r"[a-zA-Z0-9_]+").unwrap();

    let embeddings_content = fs::read_to_string(term_embeddings_path)?;
    let term_embeddings: TermEmbeddings = serde_json::from_str(&embeddings_content)?;
    let term_map = term_embeddings.0;

    let index_file = fs::File::open(index_file_path)?;
    let reader = BufReader::new(index_file);
    let all_file_paths: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    let mut global_term_counts: HashMap<String, u32> = HashMap::new();
    let mut global_pair_counts: HashMap<(String, String), u32> = HashMap::new();
    let mut global_triple_counts: HashMap<(String, String, String), u32> = HashMap::new();

    for path_str in &all_file_paths {
        let path_parts: Vec<&str> = path_str.split('/').collect();
        let file_name = path_parts.last().unwrap_or(&"").to_string();
        let path_only = path_parts[..path_parts.len() - 1].join("/");

        let mut terms_in_file: Vec<String> = Vec::new();

        for term_match in word_regex.find_iter(&path_only) {
            let term = term_match.as_str().to_lowercase();
            if term_map.contains_key(&term) {
                *global_term_counts.entry(term.clone()).or_insert(0) += 1;
                terms_in_file.push(term);
            }
        }

        for term_match in word_regex.find_iter(&file_name) {
            let term = term_match.as_str().to_lowercase();
            if term_map.contains_key(&term) {
                *global_term_counts.entry(term.clone()).or_insert(0) += 1;
                terms_in_file.push(term);
            }
        }

        let unique_terms: HashSet<String> = terms_in_file.into_iter().collect();
        let sorted_unique_terms: Vec<String> = unique_terms.into_iter().sorted().collect();

        for pair in sorted_unique_terms.iter().combinations(2) {
            let p = (pair[0].clone(), pair[1].clone());
            *global_pair_counts.entry(p.clone()).or_insert(0) += 1;
        }

        for triple in sorted_unique_terms.iter().combinations(3) {
            let t = (triple[0].clone(), triple[1].clone(), triple[2].clone());
            *global_triple_counts.entry(t.clone()).or_insert(0) += 1;
        }
    }

    let mut sorted_global_pairs: Vec<((String, String), u32)> = global_pair_counts.into_iter().collect();
    sorted_global_pairs.sort_by(|a, b| b.1.cmp(&a.1));
    let top_10_pairs: Vec<(String, String)> = sorted_global_pairs.into_iter().take(10).map(|(p, _)| p).collect();

    let mut sorted_global_triples: Vec<((String, String, String), u32)> = global_triple_counts.into_iter().collect();
    sorted_global_triples.sort_by(|a, b| b.1.cmp(&a.1));
    let top_10_triples: Vec<(String, String, String)> = sorted_global_triples.into_iter().take(10).map(|(t, _)| t).collect();

    // Print Global Term Counts
    println!("\n--- Global Term Counts ---");
    for (term, count) in &global_term_counts {
        println!("  {}: {}", term, count);
    }
    println!("--------------------------");

    println!("\n--- Top 10 Global Pairs ---");
    for pair in &top_10_pairs {
        println!("  {:?}", pair);
    }
    println!("---------------------------");

    println!("\n--- Top 10 Global Triples ---");
    for triple in &top_10_triples {
        println!("  {:?}", triple);
    }
    println!("-----------------------------");

    // Print the reports with fractions and common terms
    for report in selected_reports {
        println!("\n--- File Report: {} ---", report.file_name);
        println!("  Path Vector: {:?}", report.path_vector);
        println!("  File Name Vector: {:?}", report.file_name_vector);
        println!("  Total Vector: {:?}", report.total_vector);
        println!("  Term Counts:");
        for (term, count) in &report.term_counts {
            let global_count = global_term_counts.get(term).unwrap_or(&0);
            let fraction = *count as f64 / *global_count as f64;
            println!("    {}: {} (Fraction of Global: {:.4})", term, count, fraction);
        }

        println!("  Pairs in File:");
        for pair in &report.pairs {
            println!("    {:?}", pair);
        }

        println!("  Triples in File:");
        for triple in &report.triples {
            println!("    {:?}", triple);
        }

        // Common terms (GCD interpretation)
        println!("  Common Terms with Global (Min Count):");
        for (term, count) in &report.term_counts {
            if let Some(global_count) = global_term_counts.get(term) {
                println!("    {}: {}", term, std::cmp::min(*count, *global_count));
            }
        }
        println!("----------------------------------------");
    }

    Ok(())
}
