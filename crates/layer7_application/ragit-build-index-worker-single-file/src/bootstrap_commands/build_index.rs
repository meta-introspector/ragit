use anyhow::Result;
use std::path::PathBuf;
use sysinfo::System;
use ragit_index_types::index_struct::Index;
use super::constants::{PROMPTS_DIR_NAME, SUMMARIZE_PROMPT_FILE_NAME};
use ragit_utils::memory_utils::{print_memory_usage, check_memory_limit};
use ragit_types::build_config::BuildConfig;
use text_splitter::TextSplitter;
use std::fs;

pub fn build_index(
    verbose: bool,
    temp_dir: &PathBuf,
    _actual_root_dir: &PathBuf,
    index: &mut Index,
    _max_iterations: Option<usize>,
    sys: &mut System,
    max_memory_gb: Option<u64>,
    last_process_memory_kb: &mut Option<u64>,
) -> Result<(), anyhow::Error> {
    if verbose {
        println!("bootstrap_index_self: Running rag build");
        println!("bootstrap_index_self: Before ragit_index_effects::build (placeholder)");
        print_memory_usage(sys, "Before ragit_index_effects::build", last_process_memory_kb);
    }
    check_memory_limit(sys, max_memory_gb, "Before ragit_index_effects::build")?;

    let summary_prompt_path = temp_dir.join(PROMPTS_DIR_NAME).join(SUMMARIZE_PROMPT_FILE_NAME);
    if !summary_prompt_path.exists() {
        if verbose {
            println!("bootstrap_index_self: Warning: summarize.pdl not found, skipping prompt loading.");
        }
    }

    let build_config = BuildConfig::default();
    let splitter = TextSplitter::default();

    if verbose {
        println!("bootstrap_index_self: Iterating through staged files for chunking and indexing.");
    }
    let staged_files_cloned = index.staged_files.clone();
    for file_path_buf in &staged_files_cloned {
        let content = fs::read_to_string(file_path_buf)?;
        let chunks = splitter.chunks(&content, build_config.chunk_size);
        for chunk in chunks {
            // Here you would typically create a Chunk object and add it to the index.
            // For now, we'll just print the chunk to simulate the process.
            if verbose {
                println!("--- New Chunk ---");
                println!("{}", chunk);
            }
        }
    }

    if verbose {
        println!("bootstrap_index_self: After ragit_index_effects::build (placeholder)");
        println!("bootstrap_index_self: Built index (placeholder)");
        print_memory_usage(sys, "After ragit_index_effects::build", last_process_memory_kb);
    }
    check_memory_limit(sys, max_memory_gb, "After ragit_index_effects::build")?;
    Ok(())
}