use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use super::constants::PROMPTS_DIR_NAME;
use ragit_memory_monitor::MemoryMonitor;

pub async fn copy_prompts(
    verbose: bool,
    actual_root_dir: &PathBuf,
    temp_dir: &PathBuf,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
) -> Result<(), anyhow::Error> {
    if verbose {
        println!("bootstrap_index_self: Copying prompts");
        memory_monitor.capture_and_log_snapshot("Before copy_prompts");
    }
    memory_monitor.check_memory_limit(max_memory_gb, "Before copy_prompts")?;
    let prompts_dir = actual_root_dir.join(PROMPTS_DIR_NAME);
    let temp_prompts_dir = temp_dir.join(PROMPTS_DIR_NAME);
    fs::create_dir_all(&temp_prompts_dir)?;
    for entry in fs::read_dir(prompts_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let dest_path = temp_prompts_dir.join(path.file_name().unwrap());
            fs::copy(&path, &dest_path)?;
            if verbose {
                println!(r#"bootstrap_index_self: Copied prompt {:?} to {:?}"#, path, dest_path);
            }
            if path.file_name().unwrap() == "summarize.pdl" {
                println!("DEBUG: summarize.pdl copied to: {:?}", dest_path);
            }
        }
        memory_monitor.check_memory_limit(max_memory_gb, &format!("During copy_prompts loop for {:?}", path))?;
    }
    if verbose {
        memory_monitor.capture_and_log_snapshot("After copy_prompts");
    }
    Ok(())
}
