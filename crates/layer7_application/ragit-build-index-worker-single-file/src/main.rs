use anyhow::Result;
use std::env;
use std::fs;


mod bootstrap_commands;
use ragit_memory_monitor::MemoryMonitor;

use bootstrap_commands::setup_environment::setup_environment;
use bootstrap_commands::copy_prompts::copy_prompts;
use bootstrap_commands::add_bootstrap_files::add_bootstrap_files;
use bootstrap_commands::build_index::build_index;
use bootstrap_commands::constants::{MEMORY_USAGE_BEFORE_SETUP_ENV, MEMORY_USAGE_AFTER_SETUP_ENV, MEMORY_USAGE_BEFORE_COPY_PROMPTS, MEMORY_USAGE_AFTER_COPY_PROMPTS, CLEANUP_TEMP_DIR, BEFORE_SETUP_ENV, AFTER_SETUP_ENV, BEFORE_ADD_FILES, AFTER_ADD_FILES, MEMORY_USAGE_SUMMARY_HEADER, MEMORY_USAGE_BEFORE_BUILD_INDEX, MEMORY_USAGE_AFTER_BUILD_INDEX, BEFORE_BUILD_INDEX, AFTER_BUILD_INDEX};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let disable_cleanup = args.contains("--disable-cleanup");

    let mut memory_monitor = MemoryMonitor::new();

    println!("{}", MEMORY_USAGE_BEFORE_SETUP_ENV);
    memory_monitor.capture_and_log_snapshot(BEFORE_SETUP_ENV);

    let (actual_root_dir, temp_dir, mut index) = setup_environment(
        true, // verbose
        Some(1), // max_memory_gb (1GB for now)
        &mut memory_monitor,
    )?;

    println!("{}", MEMORY_USAGE_AFTER_SETUP_ENV);
    memory_monitor.capture_and_log_snapshot(AFTER_SETUP_ENV);

    // Call copy_prompts
    println!("{}", MEMORY_USAGE_BEFORE_COPY_PROMPTS);
    memory_monitor.capture_and_log_snapshot(BEFORE_COPY_PROMPTS);
    copy_prompts(
        true, // verbose
        &actual_root_dir,
        &temp_dir,
        Some(1), // max_memory_gb
        &mut memory_monitor,
    )?;
    println!("{}", MEMORY_USAGE_AFTER_COPY_PROMPTS);
    memory_monitor.capture_and_log_snapshot(AFTER_COPY_PROMPTS);

    // Call add_bootstrap_files
    println!("{}", MEMORY_USAGE_BEFORE_ADD_FILES);
    memory_monitor.capture_and_log_snapshot(BEFORE_ADD_FILES);
    add_bootstrap_files(
        true, // verbose
        &actual_root_dir,
        &temp_dir,
        &mut index, // index is not mutable anymore
        Some(1), // max_memory_gb
        &mut memory_monitor,
        Some(5), // max_files_to_process
    )?;
    println!("{}", MEMORY_USAGE_AFTER_ADD_FILES);
    memory_monitor.capture_and_log_snapshot(AFTER_ADD_FILES);

    // Call build_index
    println!("{}", MEMORY_USAGE_BEFORE_BUILD_INDEX);
    memory_monitor.capture_and_log_snapshot(BEFORE_BUILD_INDEX);
    build_index(
        true, // verbose
        &temp_dir,
        &actual_root_dir,
        &mut index, // index is not mutable anymore
        None, // max_iterations
        Some(1), // max_memory_gb
        &mut memory_monitor,
    )?;
    println!("{}", MEMORY_USAGE_AFTER_BUILD_INDEX);
    memory_monitor.capture_and_log_snapshot(AFTER_BUILD_INDEX);

    // Clean up the temporary directory
    if !disable_cleanup {
        fs::remove_dir_all(&temp_dir)?;
        println!("{}{:?}", CLEANUP_TEMP_DIR, temp_dir);
    } else if true { // verbose
        println!("bootstrap_index_self: Skipping cleanup of temporary directory as requested.");
    }

    // Print memory usage table
    println!("{}", MEMORY_USAGE_SUMMARY_HEADER);
    memory_monitor.print_final_report();

    Ok(())
}