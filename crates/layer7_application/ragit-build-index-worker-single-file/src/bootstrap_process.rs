use anyhow::Result;
use std::env;
use std::fs;
use std::io::Write;

use crate::bootstrap_commands::setup_environment::setup_environment;
use crate::bootstrap_commands::copy_prompts::copy_prompts;
use crate::bootstrap_commands::add_bootstrap_files::add_bootstrap_files;
use crate::bootstrap_commands::build_index::build_index;
use crate::bootstrap_commands::export_chunks::export_chunks_main;
use crate::bootstrap_commands::constants::{MEMORY_USAGE_BEFORE_SETUP_ENV, MEMORY_USAGE_AFTER_SETUP_ENV, MEMORY_USAGE_BEFORE_COPY_PROMPTS, MEMORY_USAGE_AFTER_COPY_PROMPTS, CLEANUP_TEMP_DIR, BEFORE_SETUP_ENV, AFTER_SETUP_ENV, MEMORY_USAGE_BEFORE_ADD_FILES, MEMORY_USAGE_AFTER_ADD_FILES, BEFORE_ADD_FILES, AFTER_ADD_FILES, MEMORY_USAGE_SUMMARY_HEADER, MEMORY_USAGE_BEFORE_BUILD_INDEX, MEMORY_USAGE_AFTER_BUILD_INDEX, BEFORE_BUILD_INDEX, AFTER_BUILD_INDEX, MEMORY_USAGE_BEFORE_EXPORT_CHUNKS, MEMORY_USAGE_AFTER_EXPORT_CHUNKS, BEFORE_EXPORT_CHUNKS, AFTER_EXPORT_CHUNKS};
use crate::memory_profiler::memory_monitor::MemoryMonitor;

pub async fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let disable_cleanup = args.contains("--disable-cleanup".to_string());
    let args: Vec<String> = env::args().collect();
    let disable_cleanup = args.contains("--disable-cleanup".to_string());

    let mut memory_monitor = MemoryMonitor::new();

    memory_monitor.capture_and_log_snapshot(BEFORE_SETUP_ENV);

    let (actual_root_dir, temp_dir, mut index) = setup_environment(
        false, // verbose
        None, // max_memory_gb (no limit for now)
        &mut memory_monitor,
    )?;
    println!("DEBUG: Actual root directory: {:?}", actual_root_dir);

    memory_monitor.capture_and_log_snapshot(AFTER_SETUP_ENV);

    // Call copy_prompts
    memory_monitor.capture_and_log_snapshot(BEFORE_COPY_PROMPTS);
    copy_prompts(
        false, // verbose
        &actual_root_dir,
        &temp_dir,
        None, // max_memory_gb
        &mut memory_monitor,
    )?;
    memory_monitor.capture_and_log_snapshot(AFTER_COPY_PROMPTS);

    // Call add_bootstrap_files
    memory_monitor.capture_and_log_snapshot(BEFORE_ADD_FILES);
    add_bootstrap_files(
        false, // verbose
        &actual_root_dir,
        &temp_dir,
        &mut index, // Pass mutable reference
        None, // max_memory_gb
        &mut memory_monitor,
        None, // max_files_to_process (process all files)
    )?;
    memory_monitor.capture_and_log_snapshot(AFTER_ADD_FILES);

    // Call build_index
    memory_monitor.capture_and_log_snapshot(BEFORE_BUILD_INDEX);
    build_index(
        false, // verbose
        &temp_dir,
        &actual_root_dir,
        &mut index, // Pass mutable reference
        None, // max_iterations
        None, // max_memory_gb
        &mut memory_monitor,
    )?;
    memory_monitor.capture_and_log_snapshot(AFTER_BUILD_INDEX);

    // Print memory usage table
    memory_monitor.print_final_report();

    // Count words in chunks and print to stdout
    // Print memory usage table
    memory_monitor.print_final_report();

    // Count words in chunks and print to stdout
    let word_counts = ragit_index_types::word_counter::count_words_in_chunks(&index);
    println!("\nWord Counts:\n{:?}", word_counts);

    Ok(())
}