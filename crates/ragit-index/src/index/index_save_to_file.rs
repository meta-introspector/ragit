use crate::prelude::*;

impl Index {
    pub fn save_to_file(&self, path: PathBuf) -> Result<(), Error> {
        self.save_prompts()?;

        Ok(write_bytes(
            path.to_str().unwrap(),
            &serde_json::to_vec_pretty(self)?,
            WriteMode::Atomic,
        )?)
    }
}
