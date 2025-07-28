use anyhow::Result;
use std::io::{self, Write};
use std::time::Duration;
use sysinfo::{Pid, System};
use tokio::time::timeout;

use crate::bootstrap_commands::add_bootstrap_files::add_bootstrap_files;
use crate::bootstrap_commands::build_index::build_index;
use crate::bootstrap_commands::copy_prompts::copy_prompts;
use crate::bootstrap_commands::perform_final_reflective_query::perform_final_reflective_query;
use crate::bootstrap_commands::perform_self_improvement::perform_self_improvement;
use crate::bootstrap_commands::setup_environment::setup_environment;
use crate::bootstrap_commands::write_chunks_to_markdown::write_chunks_to_markdown;

fn print_memory_usage(sys: &mut System, message: &str) {
    sys.refresh_memory();
    if let Some(process) = sys.process(Pid::from_u32(std::process::id())) {
        println!(
            "Memory Usage ({}): Total: {} KB, Used: {} KB, Process RSS: {} KB",
            message,
            sys.total_memory() / 1024,
            sys.used_memory() / 1024,
            process.memory() / 1024
        );
    } else {
        println!(
            "Memory Usage ({}): Total: {} KB, Used: {} KB",
            message,
            sys.total_memory() / 1024,
            sys.used_memory() / 1024
        );
    }
    io::stdout().flush().unwrap();
}

pub async fn bootstrap_index_self(
    verbose: bool,
    timeout_seconds: Option<u64>,
    max_iterations: Option<usize>,
) -> Result<(), anyhow::Error> {
    let mut sys = System::new_all();
    if verbose {
        println!("bootstrap_index_self: Starting");
        print_memory_usage(&mut sys, "Initial");
    }

    let bootstrap_task = async move {
        let (actual_root_dir, temp_dir, mut index) = setup_environment(verbose, &mut sys).await?;

        copy_prompts(&actual_root_dir, &temp_dir, verbose).await?;

        add_bootstrap_files(verbose, &actual_root_dir, &temp_dir, &mut index, &mut sys).await?;

        build_index(verbose, &temp_dir, &mut index, max_iterations, &mut sys).await?;

        write_chunks_to_markdown(verbose, &temp_dir, &index, &mut sys).await?;

        perform_self_improvement(verbose, &actual_root_dir, &temp_dir, &index, &mut sys).await?;

        perform_final_reflective_query(verbose, &index, &mut sys).await?;

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
