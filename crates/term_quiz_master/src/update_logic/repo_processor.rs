use std::error::Error;
use git2::Repository;
use std::collections::{HashMap, HashSet};
use crate::augmented_term_entry::AugmentedTermEntry;
use crate::cli::Args;
use crate::update_logic::commit_iterator::CommitIterator;
use crate::update_logic::commit_reporter::report_commit_progress;

pub fn process_repo_wrapper(
    repo: &Repository,
    terms_map: &mut HashMap<String, AugmentedTermEntry>,
    args: &Args,
    stopwords: &HashSet<String>,
    total_commits_in_repo: usize,
    all_reported_terms: &mut HashSet<String>,
    recently_reported_top_terms: &mut HashSet<String>,
) -> Result<(), Box<dyn Error>> {
    let mut commit_iterator = CommitIterator::new(repo, args, total_commits_in_repo)?;

    while let Some(commit_result) = commit_iterator.next() {
        let (commit, commits_processed_in_page, percentage) = commit_result?;

        report_commit_progress(
            repo,
            &commit,
            terms_map,
            commits_processed_in_page,
            total_commits_in_repo,
            percentage,
            args,
            stopwords,
            all_reported_terms,
            recently_reported_top_terms,
        )?;

        // Check for pagination limit
        if let Some(limit) = args.commits_per_page {
            if commits_processed_in_page >= limit {
                println!("Processed {} commits. Next commit to start from: {}", limit, commit.id());
                return Ok(());
            }
        }
    }
    println!("  Finished processing {} commits.", commit_iterator.commits_processed());
    Ok(())
}