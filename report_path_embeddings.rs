use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, BufRead};
use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TermEmbeddings(HashMap<String, Vec<f64>>);

#[derive(Debug, Serialize, Clone)]
struct FileReport {
    file_name: String,
    path_vector: Vec<f64>,
    file_name_vector: Vec<f64>,
    total_vector: Vec<f64>,
    term_counts: HashMap<String, u32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let term_embeddings_path = "term_embeddings.json";
    let index_file_path = "index.rs.txt";
    let embedding_dimension = 8;

    // 1. Read term_embeddings.json
    let embeddings_content = fs::read_to_string(term_embeddings_path)?;
    let term_embeddings: TermEmbeddings = serde_json::from_str(&embeddings_content)?;
    let term_map = term_embeddings.0;

    // 2. Read index.rs.txt to get file paths
    let index_file = fs::File::open(index_file_path)?;
    let reader = BufReader::new(index_file);
    let file_paths: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    let mut reports = Vec::new();

    for path_str in file_paths {
        let path_parts: Vec<&str> = path_str.split('/').collect();
        let file_name = path_parts.last().unwrap_or(&"").to_string();
        let path_only = path_parts[..path_parts.len() - 1].join("/");

        let mut path_vector = vec![0.0; embedding_dimension];
        let mut file_name_vector = vec![0.0; embedding_dimension];
        let mut total_vector = vec![0.0; embedding_dimension];
        let mut term_counts = HashMap::new();

        // Process path components
        for term in path_only.split(|c: char| !c.is_alphanumeric()) {
            if let Some(embedding) = term_map.get(&term.to_lowercase()) {
                for i in 0..embedding_dimension {
                    path_vector[i] += embedding[i];
                }
                *term_counts.entry(term.to_string()).or_insert(0) += 1;
            }
        }

        // Process file name components
        for term in file_name.split(|c: char| !c.is_alphanumeric()) {
            if let Some(embedding) = term_map.get(&term.to_lowercase()) {
                for i in 0..embedding_dimension {
                    file_name_vector[i] += embedding[i];
                }
                *term_counts.entry(term.to_string()).or_insert(0) += 1;
            }
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
            term_counts,
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

    // Print the reports
    for report in selected_reports {
        println!("{:#?}", report);
    }

    Ok(())
}