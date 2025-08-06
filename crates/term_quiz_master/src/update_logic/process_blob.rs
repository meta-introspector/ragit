use std::error::Error;
use git2::{Blob, TreeWalkResult};
use std::collections::{HashMap, HashSet};
use crate::augmented_term_entry::{AugmentedTermEntry, TermVersionInfo};
use crate::update_logic::extract_terms_from_content::extract_terms_from_content;
pub fn process_blob(
    blob: Blob,
    commit_hash: &str,
    commit_timestamp: i64,
    terms_map: &mut HashMap<String, AugmentedTermEntry>,
    new_terms_in_commit_count: &mut usize,
    current_commit_term_counts: &mut HashMap<String, usize>,
    interesting_terms_for_report: &mut Vec<String>,
    stopwords: &HashSet<String>,
) -> TreeWalkResult {
    let result: Result<(), Box<dyn Error>> = (|| {
        if let Ok(content) = String::from_utf8(blob.content().to_vec()) {
            let terms_from_file = extract_terms_from_content(&content, stopwords);
            for term in terms_from_file {
                *current_commit_term_counts.entry(term.clone()).or_insert(0) += 1;

                let entry = terms_map.entry(term.clone()).or_insert_with(|| {
                    *new_terms_in_commit_count += 1;
                    interesting_terms_for_report.push(term.clone());
                    AugmentedTermEntry {
                        term: term.clone(),
                        count: 0,
                        category: "unclassified".to_string(),
                        significance: "unclassified".to_string(),
                        vibe: "unclassified".to_string(),
                        action_suggestion: "unclassified".to_string(),
                        emoji_representation: None,
                        semantic_names: None,
                        osi_layer: None,
                        prime_factor: None,
                        is_power_of_two: None,
                        numerical_address: None,
                        embedding_vectors: None,
                        versions: Vec::new(),
                        first_seen_timestamp: Some(commit_timestamp),
                        last_seen_timestamp: Some(commit_timestamp),
                    }
                });

                // If the term already existed, update first_seen_timestamp if current commit is older
                if commit_timestamp < entry.first_seen_timestamp.unwrap() {
                    entry.first_seen_timestamp = Some(commit_timestamp);
                }

                // Add version info if not already present for this commit
                if !entry.versions.iter().any(|v| v.commit_hash == commit_hash) {
                    entry.versions.push(TermVersionInfo {
                        commit_hash: commit_hash.to_string(),
                        commit_timestamp,
                        weight: 1.0, // Initial weight
                    });
                }
                entry.count += 1;
            }
        }
        Ok(())
    })();

    match result {
        Ok(_) => TreeWalkResult::Ok,
        Err(e) => {
            eprintln!("Error processing blob: {}", e);
            TreeWalkResult::Skip
        }
    }
}