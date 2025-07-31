use anyhow::Result;
use tokio::process::Command;
use std::path::PathBuf;

pub async fn bootstrap_index_self(
    verbose: bool,
    _timeout_seconds: Option<u64>,
    max_iterations: Option<usize>,
    max_memory_gb: Option<u64>,
    max_files_to_process: Option<usize>,
    max_chunk_size: Option<usize>,
    max_summary_len: Option<usize>,
    min_summary_len: Option<usize>,
    _time_threshold_ms: Option<u128>,
    _memory_threshold_bytes: Option<u64>,
    disable_write_markdown: bool,
    disable_memory_config: bool,
    disable_prompt_copy: bool,
    disable_file_add: bool,
    disable_index_build: bool,
    disable_self_improvement: bool,
    disable_final_query: bool,
    disable_cleanup: bool,
) -> Result<(), anyhow::Error> {
    let mut cmd = Command::new(PathBuf::from("target/debug/ragit-build-index-worker-single-file"));

    if verbose {
//        cmd.arg("--verbose");
    }
    if let Some(mem_gb) = max_memory_gb {
        cmd.arg("--max-memory-gb").arg(mem_gb.to_string());
    }
    if let Some(iterations) = max_iterations {
        cmd.arg("--max-iterations").arg(iterations.to_string());
    }
    if let Some(files_to_process) = max_files_to_process {
        cmd.arg("--max-files-to-process").arg(files_to_process.to_string());
    }
    if let Some(chunk_size) = max_chunk_size {
        cmd.arg("--max-chunk-size").arg(chunk_size.to_string());
    }
    if let Some(summary_len) = max_summary_len {
        cmd.arg("--max-summary-len").arg(summary_len.to_string());
    }
    if let Some(min_len) = min_summary_len {
        cmd.arg("--min-summary-len").arg(min_len.to_string());
    }
    if disable_write_markdown {
        cmd.arg("--disable-write-markdown");
    }
    if disable_memory_config {
        cmd.arg("--disable-memory-config");
    }
    if disable_prompt_copy {
        cmd.arg("--disable-prompt-copy");
    }
    if disable_file_add {
        cmd.arg("--disable-file-add");
    }
    if disable_index_build {
        cmd.arg("--disable-index-build");
    }
    if disable_self_improvement {
        cmd.arg("--disable-self-improvement");
    }
    if disable_final_query {
        cmd.arg("--disable-final_query");
    }
    if disable_cleanup {
        cmd.arg("--disable-cleanup");
    }

    let output = cmd.output().await?;

    if !output.stdout.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }

    if !output.status.success() {
        anyhow::bail!("ragit-build-index-worker-single-file failed with status: {}", output.status);
    }

    Ok(())
}
