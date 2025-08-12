use std::collections::HashMap;
use std::fs;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
struct TermEmbeddings(HashMap<String, Vec<f64>>);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Read term_embeddings.json
    let embeddings_content = fs::read_to_string("term_embeddings.json")?;
    let term_embeddings: TermEmbeddings = serde_json::from_str(&embeddings_content)?;
    let term_map = term_embeddings.0;

    // 2. Read index.rs.txt to get file paths
    let index_file = fs::File::open("index.rs.txt")?;
    let reader = BufReader::new(index_file);
    let file_paths: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    // Regex to extract terms from paths (same as tmp_word_counter)
    let word_regex = Regex::new(r"[a-zA-Z0-9_]+").unwrap();

    // Store path embeddings
    let mut path_embeddings: HashMap<String, Vec<f64>> = HashMap::new();

    // 3. Implement path embedding logic
    for path_str in &file_paths {
        let mut path_vec = vec![0.0; 8]; // Assuming 8D embeddings
        let mut terms_found = 0;

        for mat in word_regex.find_iter(path_str) {
            let term = mat.as_str().to_lowercase();
            if let Some(embedding) = term_map.get(&term) {
                for i in 0..8 {
                    path_vec[i] += embedding[i]; // Summing embeddings
                }
                terms_found += 1;
            }
        }

        if terms_found > 0 {
            // Optionally average, but summing is fine for now as per request
            // for i in 0..8 { path_vec[i] /= terms_found as f64; }
            path_embeddings.insert(path_str.clone(), path_vec);
        }
    }

    println!("Generated embeddings for {} file paths.", path_embeddings.len());

    // 4. Implement similarity calculation
    println!("\nSearching for duplicate or very close paths...");
    let mut similar_pairs = Vec::new();
    let paths_vec: Vec<(&String, &Vec<f64>)> = path_embeddings.iter().collect();

    for i in 0..paths_vec.len() {
        for j in (i + 1)..paths_vec.len() {
            let (path1, vec1) = paths_vec[i];
            let (path2, vec2) = paths_vec[j];

            let similarity = cosine_similarity(vec1, vec2);

            if similarity > 0.95 { // Threshold for "very close" 
                similar_pairs.push((path1.clone(), path2.clone(), similarity));
            }
        }
    }

    // 5. Report duplicates/near-duplicates
    if similar_pairs.is_empty() {
        println!("No very similar file paths found (similarity > 0.95).");
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
