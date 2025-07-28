use anyhow::Result;
use std::env;
use std::fs;
use sysinfo::System;

mod bootstrap_commands;
mod memory_profiler;

use bootstrap_commands::setup_environment::setup_environment;
use bootstrap_commands::copy_prompts::copy_prompts;
use bootstrap_commands::add_bootstrap_files::add_bootstrap_files;
use bootstrap_commands::constants::{MEMORY_USAGE_BEFORE_SETUP_ENV, MEMORY_USAGE_AFTER_SETUP_ENV, MEMORY_USAGE_BEFORE_COPY_PROMPTS, MEMORY_USAGE_AFTER_COPY_PROMPTS, CLEANUP_TEMP_DIR, BEFORE_SETUP_ENV, AFTER_SETUP_ENV, BEFORE_COPY_PROMPTS, AFTER_COPY_PROMPTS, MEMORY_USAGE_BEFORE_ADD_FILES, MEMORY_USAGE_AFTER_ADD_FILES, BEFORE_ADD_FILES, AFTER_ADD_FILES, MEMORY_USAGE_SUMMARY_HEADER};
use memory_profiler::{MemorySnapshot, capture_memory_snapshot, print_memory_table};

fn main() -> Result<()> {
    let _args: Vec<String> = env::args().collect();

    let mut sys = System::new_all();
    let mut last_process_memory_kb: Option<u64> = None;
    let mut memory_snapshots: Vec<MemorySnapshot> = Vec::new();

    println!("{}", MEMORY_USAGE_BEFORE_SETUP_ENV);
    capture_memory_snapshot(BEFORE_SETUP_ENV, &mut sys, &mut last_process_memory_kb, &mut memory_snapshots);

    let (actual_root_dir, temp_dir, _index) = setup_environment(
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
        // &mut index, // index is not mutable anymore
        &mut sys,
        Some(1), // max_memory_gb
        &mut last_process_memory_kb,
        None, // max_files_to_process
    )?;
    println!("{}", MEMORY_USAGE_AFTER_ADD_FILES);
    capture_memory_snapshot(AFTER_ADD_FILES, &mut sys, &mut last_process_memory_kb, &mut memory_snapshots);

    // Call build_index
    println!("{}", MEMORY_USAGE_BEFORE_BUILD_INDEX);
    capture_memory_snapshot(BEFORE_BUILD_INDEX, &mut sys, &mut last_process_memory_kb, &mut memory_snapshots);
    build_index(
        true, // verbose
        &temp_dir,
        // &mut index, // index is not mutable anymore
        None, // max_iterations
        &mut sys,
        Some(1), // max_memory_gb
        &mut last_process_memory_kb,
    )?;
    println!("{}", MEMORY_USAGE_AFTER_BUILD_INDEX);
    capture_memory_snapshot(AFTER_BUILD_INDEX, &mut sys, &mut last_process_memory_kb, &mut memory_snapshots);

    // Clean up the temporary directory
    fs::remove_dir_all(&temp_dir)?;
    println!("{}{:?}", CLEANUP_TEMP_DIR, temp_dir);

    // Print memory usage table
    println!("{}", MEMORY_USAGE_SUMMARY_HEADER);
    print_memory_table(memory_snapshots);

    Ok(())
}