use std::error::Error;
use std::path::PathBuf;
use git2::{Repository, TreeWalkMode, TreeWalkResult};
use std::collections::{HashMap, HashSet};
use crate::augmented_term_entry::AugmentedTermEntry;
use crate::update_logic::process_blob::process_blob;

use crate::cli::Args;

pub fn process_commit(
    repo: &Repository,
    commit: &git2::Commit,
    terms_map: &mut HashMap<String, AugmentedTermEntry>,
    new_terms_in_commit_count: &mut usize,
    _args: &Args,
    stopwords: &HashSet<String>,
    all_reported_terms: &mut HashSet<String>,
) -> Result<(HashMap<String, usize>, Vec<String>), Box<dyn Error>> {
    let tree = commit.tree()?;
    let commit_hash = commit.id().to_string();
    let commit_timestamp = commit.time().seconds();

    let mut current_commit_term_counts: HashMap<String, usize> = HashMap::new();
    let mut all_new_terms_in_commit: Vec<String> = Vec::new(); // Collect all new terms

    tree.walk(TreeWalkMode::PreOrder, |root, entry| {
        let _path = PathBuf::from(root).join(entry.name().unwrap_or(""));
        if entry.kind() == Some(git2::ObjectType::Blob) {
            let blob = repo.find_blob(entry.id()).unwrap();
            process_blob(
                blob,
                &commit_hash,
                commit_timestamp,
                terms_map,
                new_terms_in_commit_count,
                &mut current_commit_term_counts,
                &mut all_new_terms_in_commit, // Pass this to process_blob
                stopwords,
            )
        } else {
            TreeWalkResult::Ok
        }
    })?;

    let mut interesting_terms_for_report: Vec<String> = Vec::new();
    let report_limit = _args.commits_per_page.unwrap_or(5); // Use commits_per_page as a proxy for report limit

    // Prioritize new terms from this commit
    for term in all_new_terms_in_commit {
        if interesting_terms_for_report.len() < report_limit && all_reported_terms.insert(term.clone()) {
            interesting_terms_for_report.push(term);
        }
    }

    // If not enough new terms, fill with other unreported terms from terms_map
    if interesting_terms_for_report.len() < report_limit {
        for (term, _) in terms_map.iter() {
            if interesting_terms_for_report.len() < report_limit && all_reported_terms.insert(term.clone()) {
                interesting_terms_for_report.push(term.clone());
            }
        }
    }

    Ok((current_commit_term_counts, interesting_terms_for_report))
}