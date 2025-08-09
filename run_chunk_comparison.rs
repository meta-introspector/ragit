use std::collections::HashMap;
use std::fs;
use std::io::{self, BufReader, BufRead};
use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
struct TermEmbeddings(HashMap<String, Vec<f64>>);

fn embed_paths_and_find_similar(
    term_embeddings_path: &str,
    index_file_path: &str,
    embedding_dimension: usize,
    similarity_threshold: f64,
    comparison_window_size: usize,
) -> Result<Vec<(String, String, f64)>, Box<dyn std::error::Error>> {
    // 1. Read term_embeddings.json
    let embeddings_content = fs::read_to_string(term_embeddings_path)?;
    let term_embeddings: TermEmbeddings = serde_json::from_str(&embeddings_content)?;
    let term_map = term_embeddings.0;

    // 2. Read index.rs.txt to get file paths
    let index_file = fs::File::open(index_file_path)?;
    let reader = BufReader::new(index_file);
    let file_paths: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    // Regex to extract terms from paths
    let word_regex = Regex::new(r"[a-zA-Z0-9_]+").unwrap();

    // Store path embeddings
    let mut path_embeddings: Vec<(String, Vec<f64>)> = Vec::new();

    // 3. Implement path embedding logic
    for path_str in &file_paths {
        let mut path_vec = vec![0.0; embedding_dimension];
        let mut terms_found = 0;

        for mat in word_regex.find_iter(path_str) {
            let term = mat.as_str().to_lowercase();
            if let Some(embedding) = term_map.get(&term) {
                for i in 0..embedding_dimension {
                    path_vec[i] += embedding[i]; // Summing embeddings
                }
                terms_found += 1;
            }
        }

        if terms_found > 0 {
            path_embeddings.push((path_str.clone(), path_vec));
        }
    }

    println!("Generated embeddings for {} file paths.", path_embeddings.len());

    // Sort path embeddings for efficient windowed comparison
    path_embeddings.sort_by(|(path1, _), (path2, _)| path1.cmp(path2));

    // 4. Implement similarity calculation with windowed comparison
    println!("\nSearching for duplicate or very close paths...");
    let mut similar_pairs = Vec::new();

    for i in 0..path_embeddings.len() {
        let (path1, vec1) = &path_embeddings[i];
        let start_j = i + 1;
        let end_j = std::cmp::min(path_embeddings.len(), i + 1 + comparison_window_size);

        for j in start_j..end_j {
            let (path2, vec2) = &path_embeddings[j];

            let similarity = cosine_similarity(vec1, vec2);

            if similarity > similarity_threshold {
                similar_pairs.push((path1.clone(), path2.clone(), similarity));
            }
        }
    }

    Ok(similar_pairs)
}

fn cosine_similarity(vec1: &[f64], vec2: &[f64]) -> f64 {
    let dot_product: f64 = vec1.iter().zip(vec2.iter()).map(|(&a, &b)| a * b).sum();
    let magnitude1: f64 = vec1.iter().map(|&a| a * a).sum::<f64>().sqrt();
    let magnitude2: f64 = vec2.iter().map(|&a| a * a).sum::<f64>().sqrt();

    if magnitude1 == 0.0 || magnitude2 == 0.0 {
        0.0
    } else {
        dot_product / (magnitude1 * magnitude2)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let term_embeddings_path = "term_embeddings.json";
    let index_file_path = "index.rs.txt";
    let embedding_dimension = 8;
    let similarity_threshold = 0.95;
    let comparison_window_size = 10;

    let similar_pairs = embed_paths_and_find_similar(
        term_embeddings_path,
        index_file_path,
        embedding_dimension,
        similarity_threshold,
        comparison_window_size,
    )?;

    // Report duplicates/near-duplicates
    if similar_pairs.is_empty() {
        println!("No very similar file paths found (similarity > {})", similarity_threshold);
    } else {
        println!("Found {} similar file paths:", similar_pairs.len());
        for (path1, path2, similarity) in similar_pairs {
            println!("  - Path 1: {}", path1);
            println!("    Path 2: {}", path2);
            println!("    Similarity: {:.4}", similarity);
            println!("--------------------------------------------------");
        }
    }

    Ok(())
}
