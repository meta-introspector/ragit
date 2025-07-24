use super::model_struct::Model;
use ragit_types::ApiError as Error;
use crate::model_raw::ModelRaw;

impl Model {
    pub fn find_api_key_in_file(&self, file_path: &str) -> Result<Option<String>, Error> {
        use std::fs::File;
        use std::io::Read;

        // Check if the file exists
        let file = match File::open(file_path) {
            Ok(file) => file,
            Err(_) => return Ok(None), // File doesn't exist or can't be opened
        };

        // Read the file content
        let mut content = String::new();
        if let Err(_) = file.take(10_000_000).read_to_string(&mut content) {
            return Ok(None); // Can't read the file
        }

        // Parse the JSON
        let models: Vec<ModelRaw> = match serde_json::from_str(&content) {
            Ok(models) => models,
            Err(_) => return Ok(None), // Can't parse the JSON
        };

        // Find the model with the same name
        for model in models {
            if model.name == self.name {
                // If the model has an API key, return it
                if let Some(key) = model.api_key {
                    return Ok(Some(key));
                }

                // If the model has an environment variable, try to get the API key from it
                if let Some(var) = model.api_env_var {
                    if let Ok(key) = std::env::var(&var) {
                        return Ok(Some(key));
                    }
                }
            }
        }

        Ok(None)
    }
}
