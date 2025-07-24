use crate::prelude::*;

impl Index {
    pub fn save_to_file(&self, path: PathBuf) -> Result<(), ApiError> {
        write_string(
            path.to_str().unwrap(),
            &serde_json::to_string_pretty(self)?,
            WriteMode::CreateOrTruncate,
        )?;

        Ok(())
    }
}