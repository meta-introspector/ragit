use super::model_struct::Model;
use ragit_types::ApiError as Error;
//use ragit_types::ApiError as Error;
use ragit_fs::join4;

impl Model {
    pub fn find_api_key_in_external_files(&self) -> Result<Option<String>, Error> {
        // Try to find the API key in the file indicated by RAGIT_MODEL_FILE
        if let Ok(file_path) = std::env::var("RAGIT_MODEL_FILE") {
            if let Some(key) = self.find_api_key_in_file(&file_path)? {
                return Ok(Some(key));
            }
        }

        // Try to find the API key in ~/.config/ragit/models.json
        if let Ok(home_dir) = std::env::var("HOME") {
            let config_path = join4(&home_dir, ".config", "ragit", "models.json")?;

            if let Some(key) = self.find_api_key_in_file(&config_path)? {
                return Ok(Some(key));
            }
        }

        Ok(None)
    }
}
