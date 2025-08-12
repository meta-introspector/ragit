use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{BufReader, BufRead};
use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;
use regex::Regex;
use itertools::Itertools;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TermEmbeddings(HashMap<String, Vec<f64>>);

#[derive(Debug, Serialize, Clone)]
struct FileReport {
    file_name: String,
    path_vector: Vec<f64>,
    file_name_vector: Vec<f64>,
    total_vector: Vec<f64>,
    term_counts: HashMap<String, u32>,
    pairs: Vec<(String, String)> ,
    triples: Vec<(String, String, String)> ,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let term_embeddings_path = "term_embeddings.json";
    let index_file_path = "index.rs.txt";
    let embedding_dimension = 8;
    let word_regex = Regex::new(r"[a-zA-Z0-9_]+").unwrap();

    // 1. Read term_embeddings.json
    let embeddings_content = fs::read_to_string(term_embeddings_path)?;
    let term_embeddings: TermEmbeddings = serde_json::from_str(&embeddings_content)?;
    let term_map = term_embeddings.0;

    // 2. Read index.rs.txt to get file paths
    let index_file = fs::File::open(index_file_path)?;
    let reader = BufReader::new(index_file);
    let all_file_paths: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    let mut reports = Vec::new();
    let mut global_term_counts: HashMap<String, u32> = HashMap::new();
    let mut global_pair_counts: HashMap<(String, String), u32> = HashMap::new();
    let mut global_triple_counts: HashMap<(String, String, String), u32> = HashMap::new();

    for path_str in &all_file_paths {
        let path_parts: Vec<&str> = path_str.split('/').collect();
        let file_name = path_parts.last().unwrap_or(&"").to_string();
        let path_only = path_parts[..path_parts.len() - 1].join("/");

        let mut path_vector = vec![0.0; embedding_dimension];
        let mut file_name_vector = vec![0.0; embedding_dimension];
        let mut total_vector = vec![0.0; embedding_dimension];
        let mut file_term_counts = HashMap::new();
        let mut terms_in_file: Vec<String> = Vec::new();

        // Process path components
        for term_match in word_regex.find_iter(&path_only) {
            let term = term_match.as_str().to_lowercase();
            if let Some(embedding) = term_map.get(&term) {
                for i in 0..embedding_dimension {
                    path_vector[i] += embedding[i];
                }
                *file_term_counts.entry(term.clone()).or_insert(0) += 1;
                *global_term_counts.entry(term.clone()).or_insert(0) += 1;
                terms_in_file.push(term);
            }
        }

        // Process file name components
        for term_match in word_regex.find_iter(&file_name) {
            let term = term_match.as_str().to_lowercase();
            if let Some(embedding) = term_map.get(&term) {
                for i in 0..embedding_dimension {
                    file_name_vector[i] += embedding[i];
                }
                *file_term_counts.entry(term.clone()).or_insert(0) += 1;
                *global_term_counts.entry(term.clone()).or_insert(0) += 1;
                terms_in_file.push(term);
            }
        }

        // Generate pairs and triples
        let mut file_pairs: Vec<(String, String)> = Vec::new();
        let mut file_triples: Vec<(String, String, String)> = Vec::new();

        let unique_terms: HashSet<String> = terms_in_file.into_iter().collect();
        let sorted_unique_terms: Vec<String> = unique_terms.into_iter().sorted().collect();

        for pair in sorted_unique_terms.iter().combinations(2) {
            let p = (pair[0].clone(), pair[1].clone());
            *global_pair_counts.entry(p.clone()).or_insert(0) += 1;
            file_pairs.push(p);
        }

        for triple in sorted_unique_terms.iter().combinations(3) {
            let t = (triple[0].clone(), triple[1].clone(), triple[2].clone());
            *global_triple_counts.entry(t.clone()).or_insert(0) += 1;
            file_triples.push(t);
        }

        // Calculate total vector
        for i in 0..embedding_dimension {
            total_vector[i] = path_vector[i] + file_name_vector[i];
        }

        reports.push(FileReport {
            file_name,
            path_vector,
            file_name_vector,
            total_vector,
            term_counts: file_term_counts,
            pairs: file_pairs,
            triples: file_triples,
        });
    }

    // Calculate magnitudes and sort
    let mut reports_with_magnitude: Vec<(FileReport, f64)> = reports
        .into_iter()
        .map(|r| {
            let magnitude = r.total_vector.iter().map(|&x| x * x).sum::<f64>().sqrt();
            (r, magnitude)
        })
        .collect();

    reports_with_magnitude.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // Select samples
    let mut selected_reports = Vec::new();
    if let Some(min_report) = reports_with_magnitude.first() {
        selected_reports.push(min_report.0.clone());
    }
    if let Some(max_report) = reports_with_magnitude.last() {
        selected_reports.push(max_report.0.clone());
    }

    let mut rng = rand::thread_rng();
    let sample_size = if reports_with_magnitude.len() > 10 { 8 } else { reports_with_magnitude.len() };
    let random_samples = reports_with_magnitude
        .choose_multiple(&mut rng, sample_size)
        .cloned()
        .map(|(r, _)| r)
        .collect::<Vec<FileReport>>();

    selected_reports.extend(random_samples);

    // Get top 10 global pairs and triples
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
