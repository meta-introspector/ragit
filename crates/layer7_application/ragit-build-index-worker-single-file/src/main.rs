use anyhow::Result;
use std::env;
use std::fs;

mod bootstrap_commands;
use ragit_memory_monitor::MemoryMonitor;

use bootstrap_commands::setup_environment::setup_environment;
use bootstrap_commands::copy_prompts::copy_prompts;
use bootstrap_commands::add_bootstrap_files::add_bootstrap_files;
use ragit_bootstrap_logic::build_index_logic::main_build_index::build_index;
use bootstrap_commands::constants::{CLEANUP_TEMP_DIR, MEMORY_USAGE_SUMMARY_HEADER};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let disable_cleanup = args.contains(&"--disable-cleanup".to_string());

    let verbose = args.contains(&"--verbose".to_string());
    let mut memory_monitor = MemoryMonitor::new(verbose, None, None);

    memory_monitor.capture_and_log_snapshot("Initial");

    let (actual_root_dir, temp_dir, mut index) = setup_environment(
        None, // max_memory_gb (no limit for now)
        &mut memory_monitor,
    ).await?;

    println!("Temporary directory: {:?}", temp_dir);

    memory_monitor.capture_and_log_snapshot("After setup_environment");

    // Call copy_prompts
    copy_prompts(
        &actual_root_dir,
        &temp_dir,
        None, // max_memory_gb
        &mut memory_monitor,
    ).await?;

    // Call add_bootstrap_files
    add_bootstrap_files(
        &actual_root_dir,
        &temp_dir,
        &mut index,
        None, // max_memory_gb
        &mut memory_monitor,
        None, // max_files_to_process (process all files)
    ).await?;

    // Call build_index
    build_index(
        &temp_dir,
        &actual_root_dir,
        &mut index,
        None, // max_iterations
        None, // max_memory_gb
        &mut memory_monitor,
    ).await?;

    // Clean up the temporary directory
    if !disable_cleanup {
        fs::remove_dir_all(&temp_dir)?;
        memory_monitor.verbose(&format!("{}{:?}", CLEANUP_TEMP_DIR, temp_dir));
    } else {
        memory_monitor.verbose("bootstrap_index_self: Skipping cleanup of temporary directory as requested.");
    }

    // Print memory usage table
    memory_monitor.verbose(MEMORY_USAGE_SUMMARY_HEADER);
    memory_monitor.print_final_report();

    Ok(())
}