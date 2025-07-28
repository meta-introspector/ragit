use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use super::constants::PROMPTS_DIR_NAME;

pub async fn copy_prompts(actual_root_dir: &PathBuf, temp_dir: &PathBuf, verbose: bool) -> Result<(), anyhow::Error> {
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
    }
    Ok(())
}
