use anyhow::Result;
use std::env;
use std::fs;
use std::io::Write;

use crate::bootstrap_commands::setup_environment::setup_environment;
use crate::bootstrap_commands::copy_prompts::copy_prompts;
use crate::bootstrap_commands::add_bootstrap_files::add_bootstrap_files;
use crate::bootstrap_commands::build_index::build_index;
use crate::bootstrap_commands::export_chunks::export_chunks_main;
use crate::bootstrap_commands::configure_memory_settings::configure_memory_settings;
use crate::bootstrap_commands::perform_self_improvement::perform_self_improvement;
use crate::bootstrap_commands::perform_final_reflective_query::perform_final_reflective_query;
use crate::bootstrap_commands::constants::{MEMORY_USAGE_BEFORE_SETUP_ENV, MEMORY_USAGE_AFTER_SETUP_ENV, MEMORY_USAGE_BEFORE_COPY_PROMPTS, MEMORY_USAGE_AFTER_COPY_PROMPTS, CLEANUP_TEMP_DIR, BEFORE_SETUP_ENV, AFTER_SETUP_ENV, MEMORY_USAGE_BEFORE_ADD_FILES, MEMORY_USAGE_AFTER_ADD_FILES, BEFORE_ADD_FILES, AFTER_ADD_FILES, MEMORY_USAGE_SUMMARY_HEADER, MEMORY_USAGE_BEFORE_BUILD_INDEX, MEMORY_USAGE_AFTER_BUILD_INDEX, BEFORE_BUILD_INDEX, AFTER_BUILD_INDEX, MEMORY_USAGE_BEFORE_EXPORT_CHUNKS, MEMORY_USAGE_AFTER_EXPORT_CHUNKS, BEFORE_EXPORT_CHUNKS, AFTER_EXPORT_CHUNKS};
use crate::memory_profiler::memory_monitor::MemoryMonitor;

pub async fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let disable_cleanup = args.contains("--disable-cleanup".to_string());
    let verbose = args.contains("--verbose");
    let max_memory_gb = args.iter().position(|arg| arg == "--max-memory-gb")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse::<u64>().ok());
    let disable_memory_config = args.contains("--disable-memory-config");
    let max_chunk_size = args.iter().position(|arg| arg == "--max-chunk-size")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse::<usize>().ok());
    let max_summary_len = args.iter().position(|arg| arg == "--max-summary-len")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse::<usize>().ok());
    let min_summary_len = args.iter().position(|arg| arg == "--min-summary-len")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse::<usize>().ok());
    let disable_write_markdown = args.contains("--disable-write-markdown");
    let max_iterations = args.iter().position(|arg| arg == "--max-iterations")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse::<usize>().ok());
    let disable_self_improvement = args.contains("--disable-self-improvement");
    let disable_final_query = args.contains("--disable-final-query");
    let target = args.iter().position(|arg| arg == "--target")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.to_string());

    let mut memory_monitor = MemoryMonitor::new(verbose, None, None); // time_threshold_ms and memory_threshold_bytes are None for now

    memory_monitor.capture_and_log_snapshot(BEFORE_SETUP_ENV);

    let (actual_root_dir, temp_dir, mut index) = setup_environment(
        verbose,
        max_memory_gb,
        &mut memory_monitor,
    ).await?;
    println!("DEBUG: Actual root directory: {:?}", actual_root_dir);

    memory_monitor.capture_and_log_snapshot(AFTER_SETUP_ENV);

    if !disable_memory_config {
        memory_monitor.check_memory_limit(max_memory_gb, "Before configure_memory_settings")?;
        
        configure_memory_settings(verbose, &mut index, max_memory_gb, max_chunk_size, max_summary_len, min_summary_len, &mut memory_monitor).await?;
    }

    // Call copy_prompts
    memory_monitor.capture_and_log_snapshot(BEFORE_COPY_PROMPTS);
    copy_prompts(
        verbose,
        &actual_root_dir,
        &temp_dir,
        max_memory_gb,
        &mut memory_monitor,
    ).await?;
    ragit_index_types::index_impl::load_prompts::load_prompts_from_directory(&mut index, &temp_dir.join("prompts"))?;
    memory_monitor.capture_and_log_snapshot(AFTER_COPY_PROMPTS);

    // Call add_bootstrap_files
    memory_monitor.capture_and_log_snapshot(BEFORE_ADD_FILES);
    add_bootstrap_files(
        verbose,
        &actual_root_dir,
        &mut index, // Pass mutable reference
        max_memory_gb,
        &mut memory_monitor,
        None, // max_files_to_process (process all files)
        target, // Pass the target argument
    ).await?;
    memory_monitor.capture_and_log_snapshot(AFTER_ADD_FILES);

    // Call build_index
    memory_monitor.capture_and_log_snapshot(BEFORE_BUILD_INDEX);
    build_index(
        verbose,
        &temp_dir,
        &actual_root_dir,
        &mut index, // Pass mutable reference
        max_iterations,
        max_memory_gb,
        &mut memory_monitor,
    ).await?;
    memory_monitor.capture_and_log_snapshot(AFTER_BUILD_INDEX);

    if !disable_write_markdown {
        memory_monitor.check_memory_limit(max_memory_gb, "Before write_chunks_to_markdown")?;
        
        export_chunks_main::write_chunks_to_markdown(verbose, &temp_dir, &index, max_memory_gb, &mut memory_monitor, max_iterations).await?;
    } else {
        memory_monitor.verbose("bootstrap_index_self: Skipping writing chunks to markdown as requested.");
    }

    if !disable_self_improvement {
        memory_monitor.check_memory_limit(max_memory_gb, "Before perform_self_improvement")?;
        
        perform_self_improvement(verbose, &actual_root_dir, &temp_dir, &index, max_memory_gb, &mut memory_monitor).await?;
    }

    if !disable_final_query {
        memory_monitor.check_memory_limit(max_memory_gb, "Before perform_final_reflective_query")?;
        
        perform_final_reflective_query(verbose, &index, max_memory_gb, &mut memory_monitor).await?;
    }

    // Print memory usage table
    memory_monitor.print_final_report();

    // Count words in chunks and print to stdout
    // Print memory usage table
    memory_monitor.print_final_report();

    // Count words in chunks and print to stdout
    let word_counts = ragit_index_types::word_counter::count_words_in_chunks(&index);
    println!("\nWord Counts:\n{:?}", word_counts);

    // Clean up the temporary directory
    if !disable_cleanup {
        fs::remove_dir_all(&temp_dir)?;
    }

    Ok(())
}
