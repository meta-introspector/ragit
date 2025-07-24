use crate::prelude::*;

impl Index {
    pub fn load_or_init_prompts(&mut self) -> Result<(), ApiError> {
        for prompt_name in PROMPTS.keys() {
            let prompt_path = get_rag_path(
                &self.root_dir,
                &join_paths(
                    PROMPT_DIR_NAME,
                    &format!("{}.pdl", prompt_name),
                )?,
            )?;

            if !exists(&prompt_path) {
                write_string(
                    &prompt_path.to_str().unwrap(),
                    PROMPTS.get(prompt_name).unwrap().to_string(),
                    WriteMode::Create,
                )?;
            }
        }

        Ok(())
    }

    pub fn save_prompts(&self) -> Result<(), ApiError> {
        let prompt_real_dir = get_rag_path(&self.root_dir, PROMPT_DIR_NAME)?;

        for prompt_name in PROMPTS.keys() {
            let prompt_path = get_rag_path(
                &self.root_dir,
                &join_paths(
                    PROMPT_DIR_NAME,
                    &format!("{}.pdl", prompt_name),
                )?,
            )?;

            write_string(
                &prompt_path.to_str().unwrap(),
                PROMPTS.get(prompt_name).unwrap().to_string(),
                WriteMode::CreateOrTruncate,
            )?;
        }

        Ok(())
    }
}