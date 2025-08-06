use std::error::Error;
use git2::Repository;
use std::collections::HashMap;
use crate::augmented_term_entry::AugmentedTermEntry;
use crate::cli::Args;
use crate::update_logic::process_commit;
use std::collections::HashSet;

pub fn report_commit_progress(
    repo: &Repository,
    commit: &git2::Commit,
    terms_map: &mut HashMap<String, AugmentedTermEntry>,
    commits_processed_in_page: usize,
    total_commits_in_repo: usize,
    percentage: f64,
    args: &Args,
    stopwords: &HashSet<String>,
    all_reported_terms: &mut HashSet<String>,
    recently_reported_top_terms: &mut HashSet<String>,
) -> Result<(), Box<dyn Error>> {
    let mut new_terms_in_commit_count = 0;
    let (current_commit_term_counts, all_new_terms_in_commit) = process_commit::process_commit(
        repo,
        &commit,
        terms_map,
        &mut new_terms_in_commit_count,
        args,
        stopwords,
        all_reported_terms,
    )?;

    let mut interesting_terms_for_report: Vec<String> = Vec::new();
    for term in all_new_terms_in_commit {
        if all_reported_terms.insert(term.clone()) {
            interesting_terms_for_report.push(term);
        }
    }

    let mut top_term_in_commit: Option<(&String, &usize)> = None;
    let mut sorted_commit_terms: Vec<(&String, &usize)> = current_commit_term_counts.iter().collect();
    sorted_commit_terms.sort_by(|a, b| b.1.cmp(a.1)); // Sort by count descending

    // Try to find a top term that hasn't been recently reported
    for (term, count) in &sorted_commit_terms {
        if !stopwords.contains(term.as_str()) && !recently_reported_top_terms.contains(term.as_str()) {
            top_term_in_commit = Some((term, count));
            break; // Found a suitable top term
        }
    }

    // If all top terms have been recently reported, fall back to the absolute top
    if top_term_in_commit.is_none() && !sorted_commit_terms.is_empty() {
        for (term, count) in &sorted_commit_terms {
            if !stopwords.contains(term.as_str()) {
                top_term_in_commit = Some((term, count));
                break;
            }
        }
    }

    let top_term_info = if let Some((term, count)) = top_term_in_commit {
        // Add the selected top term to the recently reported set
        recently_reported_top_terms.insert(term.clone());
        format!("Top: \"{}\" ({})", term, count)
    } else {
        "Top: N/A".to_string()
    };

    // Construct the output line, ensuring it fits within 80 characters
    let mut output_line = format!("  {}/{} ({:.2}%) | New: {} | {}",
                                 commits_processed_in_page, total_commits_in_repo, percentage,
                                 new_terms_in_commit_count, top_term_info);

    // Add interesting terms if there's space
    let mut current_len = output_line.len();
    for term in &interesting_terms_for_report {
        let term_str = format!(" {}", term);
        if current_len + term_str.len() <= 80 {
            output_line.push_str(&term_str);
            current_len += term_str.len();
        } else {
            break;
        }
    }
    println!("{}", output_line);

    Ok(())
}
