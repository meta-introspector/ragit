use crate::constant::PROMPT_DIR_NAME;
use crate::error::Error;
use crate::prompts::PROMPTS;
use crate::path_utils::{get_rag_path, join_paths, pathbuf_to_str, str_to_pathbuf};
use ragit_fs::{create_dir_all, read_string, set_extension, write_string, WriteMode};



use crate::index::index_struct::Index;

impl Index {
    // `Index::load` calls this function. There's no need to call this again.
    pub fn load_or_init_prompts(&mut self) -> Result<(), Error> {
        let mut has_inited_prompt = false;

        for prompt_name in PROMPTS.keys() {
            let prompt_path = get_rag_path(
                &self.root_dir,
                &join_paths(
                    &str_to_pathbuf(PROMPT_DIR_NAME),
                    &str_to_pathbuf(&set_extension(
                        prompt_name,
                        "pdl",
                    )?),
                )?,
            )?;

            match read_string(prompt_path.to_str().unwrap()) {
                Ok(p) => {
                    self.prompts.insert(prompt_name.to_string(), p);
                },
                Err(_) => {
                    eprintln!("Warning: failed to load `{prompt_name}.pdl`");
                    self.prompts.insert(prompt_name.to_string(), PROMPTS.get(prompt_name).unwrap().to_string());
                    has_inited_prompt = true;
                },
            }
        }

        if has_inited_prompt {
            self.save_prompts()?;
        }

        Ok(())
    }

    pub fn save_prompts(&self) -> Result<(), Error> {
        let prompt_real_dir = get_rag_path(
            &self.root_dir,
            &str_to_pathbuf(PROMPT_DIR_NAME),
        )?;

        if !prompt_real_dir.exists() {
            create_dir_all(&pathbuf_to_str(&prompt_real_dir))?;
        }

        for (prompt_name, prompt) in self.prompts.iter() {
            let prompt_path = join_paths(
                &prompt_real_dir,
                &str_to_pathbuf(&set_extension(
                    prompt_name,
                    "pdl",
                )?),
            )?;

            write_string(
                prompt_path.to_str().unwrap(),
                prompt,
                WriteMode::Atomic,
            )?;
        }

        Ok(())
    }

    /// It does NOT save the prompt to the file. You have to run `save_prompts` to save it.
    /// `key` is a name of the prompt, like `extract_keyword`, not `extract_keyword.pdl`.
    /// `value` is a content of a pdl file.
    pub fn update_prompt(&mut self, key: String, value: String) {
        self.prompts.insert(key, value);
    }
}
