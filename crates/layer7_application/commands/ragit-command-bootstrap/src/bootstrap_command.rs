use anyhow::Result;
use std::time::Duration;
use sysinfo::System;
use tokio::time::timeout;

use crate::bootstrap_commands::add_bootstrap_files::add_bootstrap_files;
use crate::bootstrap_commands::build_index::build_index;
use crate::bootstrap_commands::copy_prompts::copy_prompts;
use crate::bootstrap_commands::perform_final_reflective_query::perform_final_reflective_query;
use crate::bootstrap_commands::perform_self_improvement::perform_self_improvement;
use crate::bootstrap_commands::setup_environment::setup_environment;
use crate::bootstrap_commands::write_chunks_to_markdown::write_chunks_to_markdown_main::write_chunks_to_markdown;
use crate::bootstrap_commands::configure_memory_settings::configure_memory_settings;
use ragit_utils::memory_utils::{print_memory_usage, check_memory_limit, print_process_list};

pub async fn bootstrap_index_self(
    verbose: bool,
    timeout_seconds: Option<u64>,
    max_iterations: Option<usize>,
    max_memory_gb: Option<u64>,
    max_files_to_process: Option<usize>,
    disable_write_markdown: bool,
    disable_memory_config: bool,
    disable_prompt_copy: bool,
    disable_file_add: bool,
    disable_index_build: bool,
    disable_self_improvement: bool,
    disable_final_query: bool,
) -> Result<(), anyhow::Error> {
    let max_iterations = max_iterations.or(Some(1));
    let max_files_to_process = max_files_to_process.or(Some(1));
    let disable_write_markdown = disable_write_markdown || true; // Ensure it's true if not explicitly false
    let max_memory_gb = max_memory_gb.or(Some(1));
    let mut sys = System::new_all();
    let mut last_process_memory_kb: Option<u64> = None;

    if verbose {
        println!("bootstrap_index_self: Starting");
        print_memory_usage(&mut sys, "Initial", &mut last_process_memory_kb);
        print_process_list(&mut sys, "Initial");
    }

    let bootstrap_task = async move {
        check_memory_limit(&mut sys, max_memory_gb, "Before setup_environment")?;
        print_process_list(&mut sys, "Before setup_environment");
        let (actual_root_dir, temp_dir, mut index) = setup_environment(verbose, &mut sys, max_memory_gb, &mut last_process_memory_kb).await?;

        if !disable_memory_config {
            check_memory_limit(&mut sys, max_memory_gb, "Before configure_memory_settings")?;
            print_process_list(&mut sys, "Before configure_memory_settings");
            configure_memory_settings(verbose, &mut index, &mut sys, max_memory_gb, &mut last_process_memory_kb).await?;
        }

        if !disable_prompt_copy {
            check_memory_limit(&mut sys, max_memory_gb, "Before copy_prompts")?;
            print_process_list(&mut sys, "Before copy_prompts");
            copy_prompts(verbose, &actual_root_dir, &temp_dir, &mut sys, max_memory_gb, &mut last_process_memory_kb).await?;
            ragit_index_types::index_impl::load_prompts::load_prompts_from_directory(&mut index, &temp_dir.join("prompts"))?;
        }

        if !disable_file_add {
            check_memory_limit(&mut sys, max_memory_gb, "Before add_bootstrap_files")?;
            print_process_list(&mut sys, "Before add_bootstrap_files");
            add_bootstrap_files(verbose, &actual_root_dir, &temp_dir, &mut index, &mut sys, max_memory_gb, &mut last_process_memory_kb, max_files_to_process).await?;
        }

        if !disable_index_build {
            check_memory_limit(&mut sys, max_memory_gb, "Before build_index")?;
            print_process_list(&mut sys, "Before build_index");
            build_index(verbose, &temp_dir, &mut index, max_iterations, &mut sys, max_memory_gb, &mut last_process_memory_kb).await?;
        }

        if !disable_write_markdown {
            check_memory_limit(&mut sys, max_memory_gb, "Before write_chunks_to_markdown")?;
            print_process_list(&mut sys, "Before write_chunks_to_markdown");
            write_chunks_to_markdown(verbose, &temp_dir, &index, &mut sys, max_memory_gb, &mut last_process_memory_kb, max_iterations).await?;
        } else if verbose {
            println!("bootstrap_index_self: Skipping writing chunks to markdown as requested.");
        }

        if !disable_self_improvement {
            check_memory_limit(&mut sys, max_memory_gb, "Before perform_self_improvement")?;
            print_process_list(&mut sys, "Before perform_self_improvement");
            perform_self_improvement(verbose, &actual_root_dir, &temp_dir, &index, &mut sys, max_memory_gb, &mut last_process_memory_kb).await?;
        }

        if !disable_final_query {
            check_memory_limit(&mut sys, max_memory_gb, "Before perform_final_reflective_query")?;
            print_process_list(&mut sys, "Before perform_final_reflective_query");
            perform_final_reflective_query(verbose, &index, &mut sys, max_memory_gb, &mut last_process_memory_kb).await?;
        }

        check_memory_limit(&mut sys, max_memory_gb, "Before final return")?;
        print_process_list(&mut sys, "Final");
        Ok(())
    };

    match timeout_seconds {
        Some(seconds) => {
            match timeout(Duration::from_secs(seconds), bootstrap_task).await {
                Ok(result) => result,
                Err(_) => Err(anyhow::anyhow!(
                    "Bootstrap operation timed out after {} seconds",
                    seconds
                )),
            }
        }
        None => bootstrap_task.await,
    }
}
