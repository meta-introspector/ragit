use anyhow::Result;
use std::env;
use std::fs;
use sysinfo::System;

use crate::bootstrap_commands::setup_environment::setup_environment;
use crate::bootstrap_commands::copy_prompts::copy_prompts;
use crate::bootstrap_commands::add_bootstrap_files::add_bootstrap_files;
use crate::bootstrap_commands::build_index::build_index;
use crate::bootstrap_commands::constants::{MEMORY_USAGE_BEFORE_SETUP_ENV, MEMORY_USAGE_AFTER_SETUP_ENV, MEMORY_USAGE_BEFORE_COPY_PROMPTS, MEMORY_USAGE_AFTER_COPY_PROMPTS, CLEANUP_TEMP_DIR, BEFORE_SETUP_ENV, AFTER_SETUP_ENV, BEFORE_COPY_PROMPTS, AFTER_COPY_PROMPTS, MEMORY_USAGE_BEFORE_ADD_FILES, MEMORY_USAGE_AFTER_ADD_FILES, BEFORE_ADD_FILES, AFTER_ADD_FILES, MEMORY_USAGE_SUMMARY_HEADER, MEMORY_USAGE_BEFORE_BUILD_INDEX, MEMORY_USAGE_AFTER_BUILD_INDEX, BEFORE_BUILD_INDEX, AFTER_BUILD_INDEX};
use crate::memory_profiler::{MemorySnapshot, capture_memory_snapshot, print_memory_table};

pub async fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let disable_cleanup = args.contains(&"--disable-cleanup".to_string());
    let args: Vec<String> = env::args().collect();
    let disable_cleanup = args.contains(&"--disable-cleanup".to_string());

    let mut sys = System::new_all();
    let mut last_process_memory_kb: Option<u64> = None;
    let mut memory_snapshots: Vec<MemorySnapshot> = Vec::new();

    println!("{}", MEMORY_USAGE_BEFORE_SETUP_ENV);
    capture_memory_snapshot(BEFORE_SETUP_ENV, &mut sys, &mut last_process_memory_kb, &mut memory_snapshots);

    let (actual_root_dir, temp_dir, mut index) = setup_environment(
        true, // verbose
        &mut sys,
        Some(1), // max_memory_gb (1GB for now)
        &mut last_process_memory_kb,
    )?;

    println!("{}", MEMORY_USAGE_AFTER_SETUP_ENV);
    capture_memory_snapshot(AFTER_SETUP_ENV, &mut sys, &mut last_process_memory_kb, &mut memory_snapshots);

    // Call copy_prompts
    println!("{}", MEMORY_USAGE_BEFORE_COPY_PROMPTS);
    capture_memory_snapshot(BEFORE_COPY_PROMPTS, &mut sys, &mut last_process_memory_kb, &mut memory_snapshots);
    copy_prompts(
        true, // verbose
        &actual_root_dir,
        &temp_dir,
        &mut sys,
        Some(1), // max_memory_gb
        &mut last_process_memory_kb,
    )?;
    println!("{}", MEMORY_USAGE_AFTER_COPY_PROMPTS);
    capture_memory_snapshot(AFTER_COPY_PROMPTS, &mut sys, &mut last_process_memory_kb, &mut memory_snapshots);

    // Call add_bootstrap_files
    println!("{}", MEMORY_USAGE_BEFORE_ADD_FILES);
    capture_memory_snapshot(BEFORE_ADD_FILES, &mut sys, &mut last_process_memory_kb, &mut memory_snapshots);
    add_bootstrap_files(
        true, // verbose
        &actual_root_dir,
        &temp_dir,
        &mut index, // index is not mutable anymore
        &mut sys,
        Some(1), // max_memory_gb
        &mut last_process_memory_kb,
        Some(5), // max_files_to_process
    )?;
    println!("{}", MEMORY_USAGE_AFTER_ADD_FILES);
    capture_memory_snapshot(AFTER_ADD_FILES, &mut sys, &mut last_process_memory_kb, &mut memory_snapshots);

    // Call build_index
    println!("{}", MEMORY_USAGE_BEFORE_BUILD_INDEX);
    capture_memory_snapshot(BEFORE_BUILD_INDEX, &mut sys, &mut last_process_memory_kb, &mut memory_snapshots);
    build_index(
        true, // verbose
        &temp_dir,
        &actual_root_dir,
        &mut index, // index is not mutable anymore
        None, // max_iterations
        &mut sys,
        Some(1), // max_memory_gb
        &mut last_process_memory_kb,
    )?;
    println!("{}", MEMORY_USAGE_AFTER_BUILD_INDEX);
    capture_memory_snapshot(AFTER_BUILD_INDEX, &mut sys, &mut last_process_memory_kb, &mut memory_snapshots);

    // Clean up the temporary directory
    if !disable_cleanup {
        fs::remove_dir_all(&temp_dir)?;
        println!("{}{:?}", CLEANUP_TEMP_DIR, temp_dir);
    } else if true { // verbose
        println!("bootstrap_index_self: Skipping cleanup of temporary directory as requested.");
    }

    // Print memory usage table
    println!("{}", MEMORY_USAGE_SUMMARY_HEADER);
    print_memory_table(memory_snapshots);

    Ok(())
}