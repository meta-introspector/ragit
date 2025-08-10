use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{BufReader, BufRead};
use serde::{Deserialize, Serialize};
use rand::prelude::IndexedRandom;
use regex::Regex;
use itertools::Itertools;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermEmbeddings(HashMap<String, Vec<f64>>);

#[derive(Debug, Serialize, Clone)]
pub struct FileReport {
    pub file_name: String,
    pub path_vector: Vec<f64>,
    pub file_name_vector: Vec<f64>,
    pub total_vector: Vec<f64>,
    pub term_counts: HashMap<String, u32>,
    pub pairs: Vec<(String, String)> ,
    pub triples: Vec<(String, String, String)> ,
}

pub fn get_all_file_reports() -> Result<Vec<FileReport>, Box<dyn std::error::Error>> {
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

    Ok(reports)
}