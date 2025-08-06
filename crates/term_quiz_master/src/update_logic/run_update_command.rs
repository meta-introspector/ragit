use std::error::Error;
use std::path::PathBuf;
use git2::Repository;
use std::collections::{HashMap, HashSet};
use serde::Deserialize;
use toml;
use crate::cli::Args;
use crate::cache::CommitCountCache;
use crate::update_logic::config_loader;
use crate::update_logic::repo_processor;
use crate::update_logic::finalizer;
use crate::augmented_term_entry::AugmentedTermEntry;

// #[derive(Debug, Deserialize)]
// struct StopwordsConfig {
//     stopwords: Vec<String>,
// }

pub fn run_update_command(args: &Args) -> Result<(), Box<dyn Error>> {
    println!("Running update command...");

    let repo_path = PathBuf::from("/data/data/com.termux/files/home/storage/github/ragit");
    let repo = Repository::open(&repo_path)?;

    let stopwords = config_loader::load_stopwords(&repo_path)?;

    let mut terms_map: HashMap<String, AugmentedTermEntry> = HashMap::new();
    let mut all_reported_terms: HashSet<String> = HashSet::new();
    let mut recently_reported_top_terms: HashSet<String> = HashSet::new(); // NEW

    process_main_repo_logic(&repo, &mut terms_map, args, &stopwords, &mut all_reported_terms, &mut recently_reported_top_terms)?;

    process_submodules_logic(&repo, &repo_path, &mut terms_map, args, &stopwords, &mut all_reported_terms, &mut recently_reported_top_terms)?;

    let base_dir = PathBuf::from("/data/data/com.termux/files/home/storage/github/ragit/index/solfunmeme-index");
    finalizer::finalize_index(terms_map, &base_dir)?;

    Ok(())
}

fn process_main_repo_logic(
    repo: &Repository,
    terms_map: &mut HashMap<String, AugmentedTermEntry>,
    args: &Args,
    stopwords: &HashSet<String>,
    all_reported_terms: &mut HashSet<String>,
    recently_reported_top_terms: &mut HashSet<String>,
) -> Result<(), Box<dyn Error>> {
    // Determine total commits for the main repository (cached or calculated)
    let head_commit_oid = repo.head()?.resolve()?.peel_to_commit()?.id();
    let head_hash = head_commit_oid.to_string();

    let mut total_commits_in_main_repo = 0;
    let mut total_commits_source = "unknown";

    if let Ok(Some(cached_count)) = CommitCountCache::load() {
        if cached_count.head_hash == head_hash {
            total_commits_in_main_repo = cached_count.total_commits;
            total_commits_source = "cached";
        }
    }

    if total_commits_source == "unknown" {
        // Calculate total commits if not cached or cache is stale
        let mut temp_revwalk = repo.revwalk()?;
        temp_revwalk.push_head()?;
        total_commits_in_main_repo = temp_revwalk.count();
        total_commits_source = "calculated";

        // Save to cache
        let cache = CommitCountCache { head_hash: head_hash.clone(), total_commits: total_commits_in_main_repo };
        if let Err(e) = cache.save() {
            eprintln!("Warning: Could not save commit count cache: {}", e);
        }
    }
    println!("Total commits in repository ({}) {}", total_commits_source, total_commits_in_main_repo);

    repo_processor::process_repo_wrapper(repo, terms_map, args, stopwords, total_commits_in_main_repo, all_reported_terms, recently_reported_top_terms)?;

    Ok(())
}

fn process_submodules_logic(
    repo: &Repository,
    repo_path: &PathBuf,
    terms_map: &mut HashMap<String, AugmentedTermEntry>,
    args: &Args,
    stopwords: &HashSet<String>,
    all_reported_terms: &mut HashSet<String>,
    recently_reported_top_terms: &mut HashSet<String>,
) -> Result<(), Box<dyn Error>> {
    // Process submodules if --recurse-submodules is enabled and --no-submodules is not set
    if args.recurse_submodules && !args.no_submodules {
        for submodule in repo.submodules()? {
            let submodule_path = repo_path.join(submodule.path());
            if submodule_path.exists() {
                let submodule_repo = Repository::open(&submodule_path)?;
                // For submodules, we don't have a cached total count, so pass 0 for now.
                // The percentage will be based on commits processed within the submodule.
                repo_processor::process_repo_wrapper(&submodule_repo, terms_map, args, stopwords, 0, all_reported_terms, recently_reported_top_terms)?;
            } else {
                println!("Submodule path does not exist: {:?}", submodule_path);
            }
        }
    }
    Ok(())
}