use ragit_index::Index;
use anyhow::Result;
use ragit_types::ApiError;
use ragit_fs::{exists, write_string, WriteMode};
use ragit_utils::ragit_path_utils::get_rag_path;
use ragit_utils::constant::PROMPT_DIR_NAME;
use ragit_utils::prompts::PROMPTS;
use std::path::PathBuf;

pub fn load_or_init_prompts(index: &mut Index) -> Result<(), ApiError> {
    for prompt_name in PROMPTS.keys() {
        let prompt_path = get_rag_path(
            &index.root_dir,
            &PathBuf::from(format!("{}/{}.pdl", PROMPT_DIR_NAME, prompt_name)),
        ).map_err(|e| ApiError::from(anyhow::Error::new(e)))?;

        if !exists(&prompt_path) {
            write_string(
                &prompt_path.to_str().unwrap(),
                PROMPTS.get(prompt_name).unwrap(),
                WriteMode::AlwaysCreate,
            ).map_err(|e| ApiError::from(anyhow::Error::new(e)))?;
        }
    }

    Ok(())
}

pub fn save_prompts(index: &Index) -> Result<(), ApiError> {
    let prompt_real_dir = get_rag_path(&index.root_dir, &PathBuf::from(PROMPT_DIR_NAME))
        .map_err(|e| ApiError::from(anyhow::Error::new(e)))?;

    for prompt_name in PROMPTS.keys() {
        let prompt_path = get_rag_path(
            &index.root_dir,
            &PathBuf::from(format!("{}/{}.pdl", PROMPT_DIR_NAME, prompt_name)),
        ).map_err(|e| ApiError::from(anyhow::Error::new(e)))?;

        write_string(
            &prompt_path.to_str().unwrap(),
            PROMPTS.get(prompt_name).unwrap(),
            WriteMode::CreateOrTruncate,
        ).map_err(|e| ApiError::from(anyhow::Error::new(e)))?;
    }

    Ok(())
}
