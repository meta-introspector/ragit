use std::fs;
use std::collections::HashMap;
use serde_json;

fn main() {
    let embeddings_path = format!("{}/../../term_embeddings.json", env!("CARGO_MANIFEST_DIR"));
    let embeddings_str = fs::read_to_string(&embeddings_path).expect("Could not read term_embeddings.json");
    let raw_embeddings: HashMap<String, Vec<f32>> = serde_json::from_str(&embeddings_str).expect("Could not parse term_embeddings.json");

    let mut potential_positive_pairs = Vec::new();

    for (term, _) in raw_embeddings.iter() {
        if term.contains('_') {
            for sub_term in term.split('_') {
                if raw_embeddings.contains_key(sub_term) {
                    potential_positive_pairs.push((term.clone(), sub_term.to_string()));
                }
            }
        }
    }

    println!("Potential Positive Pairs (Term, Sub-term):");
    for (term, sub_term) in potential_positive_pairs {
        println!("(\"{}\", \"{}\")", term, sub_term);
    }
}
