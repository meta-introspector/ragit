use anyhow::Result;
use std::path::PathBuf;
use tokio::process::Command;

use ragit_index_types::index_struct::Index;
use super::log_start::log_start;
use super::get_self_code::get_self_code;
use super::format_prompt::format_prompt;
use super::execute_query::execute_query;
use super::handle_improved_code::handle_improved_code;
use ragit_memory_monitor::MemoryMonitor;

pub async fn run_self_improvement_loop(
    verbose: bool,
    actual_root_dir: &PathBuf,
    temp_dir: &PathBuf,
    index: &Index,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
    max_iterations: Option<usize>,
) -> Result<(), anyhow::Error> {
    for i in 0..max_iterations.unwrap_or(1) {
        memory_monitor.verbose(&format!("--- Self-Improvement Iteration {} ---", i + 1));
        log_start(verbose, memory_monitor);
        memory_monitor.verbose("perform_self_improvement: Log start completed.");
        memory_monitor.check_memory_limit(max_memory_gb, "Before get_self_code")?;
        let self_code = get_self_code(actual_root_dir, memory_monitor)?;
        memory_monitor.verbose(&format!(
            "perform_self_improvement: Retrieved self code ({} bytes).",
            self_code.len()
        ));
        memory_monitor.check_memory_limit(max_memory_gb, "After get_self_code")?;
        let prompt = format_prompt(&self_code, memory_monitor);
        memory_monitor.verbose("perform_self_improvement: Formatted prompt.");
        memory_monitor.check_memory_limit(
            max_memory_gb,
            "Before execute_query (self-improvement)",
        )?;
        let improved_code =
            execute_query(verbose, index, &prompt, max_memory_gb, memory_monitor).await?;
        memory_monitor.verbose(&format!(
            "perform_self_improvement: Executed self-improvement query. Response length: {}.",
            improved_code.len()
        ));
        memory_monitor.check_memory_limit(max_memory_gb, "After execute_query (self-improvement)")?;
        handle_improved_code(verbose, temp_dir, &improved_code, memory_monitor)?;
        memory_monitor.verbose("perform_self_improvement: Handled improved code.");
        memory_monitor.check_memory_limit(max_memory_gb, "After handle_improved_code")?;

        // Compile the improved code
        let compile_success = compile_improved_code(temp_dir, memory_monitor).await?;
        if compile_success {
            memory_monitor.verbose("Self-improvement: Compilation successful.");
            // Test the improved code
            let test_success = test_improved_code(temp_dir, memory_monitor).await?;
            if test_success {
                memory_monitor.verbose("Self-improvement: Tests passed.");
            } else {
                memory_monitor.verbose("Self-improvement: Tests failed.");
            }
        } else {
            memory_monitor.verbose("Self-improvement: Compilation failed.");
        }

        // TODO: Add logic to evaluate the LLM's output and decide whether to continue.
    }
    Ok(())
}

async fn compile_improved_code(
    temp_dir: &PathBuf,
    memory_monitor: &mut MemoryMonitor,
) -> Result<bool, anyhow::Error> {
    memory_monitor.verbose(&format!("Attempting to compile improved code in {:?}", temp_dir));
    let output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(temp_dir)
        .output()
        .await?;

    if !output.stdout.is_empty() {
        memory_monitor.verbose(&format!("Compile stdout: {}", String::from_utf8_lossy(&output.stdout)));
    }
    if !output.stderr.is_empty() {
        memory_monitor.verbose(&format!("Compile stderr: {}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(output.status.success())
}

async fn test_improved_code(
    temp_dir: &PathBuf,
    memory_monitor: &mut MemoryMonitor,
) -> Result<bool, anyhow::Error> {
    memory_monitor.verbose(&format!("Attempting to run tests on improved code in {:?}", temp_dir));
    let output = Command::new("cargo")
        .arg("test")
        .current_dir(temp_dir)
        .output()
        .await?;

    if !output.stdout.is_empty() {
        memory_monitor.verbose(&format!("Test stdout: {}", String::from_utf8_lossy(&output.stdout)));
    }
    if !output.stderr.is_empty() {
        memory_monitor.verbose(&format!("Test stderr: {}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(output.status.success())
}
