use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::index_struct::Index;

pub fn load_prompts_from_directory(index: &mut Index, prompts_dir: &PathBuf) -> Result<()> {
    for entry in fs::read_dir(prompts_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "pdl") {
            let prompt_name = path.file_stem().unwrap().to_string_lossy().into_owned();
            let prompt_content = fs::read_to_string(&path)?;
            index.prompts.insert(prompt_name, prompt_content);
        }
    }
    Ok(())
}
