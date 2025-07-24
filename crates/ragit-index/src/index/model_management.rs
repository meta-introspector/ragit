use crate::prelude::*;

impl Index {
    pub fn load_or_init_models(&mut self) -> Result<(), Error> {
        let models_at = get_rag_path(&self.root_dir, &PathBuf::from(MODEL_FILE_NAME))?;

        if !exists(&models_at) {
            // Initialize models from an external source or defaults
            let models = Index::get_initial_models()?;

            // Always ensure API keys are null in the local models file
            let models_without_api_keys = self.remove_api_keys_from_models(models);

            // Write the models to the local file
            write_string(
                models_at.to_str().unwrap(),
                &serde_json::to_string_pretty(&models_without_api_keys)?,
                WriteMode::Atomic,
            )?;
        }

        // Load models from the local file
        let j = read_string(models_at.to_str().unwrap())?;
        let models = serde_json::from_str::<Vec<ModelRaw>>(&j)?;
        let mut result = vec![];

        for model in models.iter() {
            result.push(Model::try_from(model)?);
        }

        self.models = result;
        Ok(())
    }

    // Get initial models from environment variable, config file, or defaults
    pub fn get_initial_models() -> Result<Vec<ModelRaw>, Error> {
        // Check for environment variable RAGIT_MODEL_CONFIG
        if let Ok(env_path_str) = std::env::var("RAGIT_MODEL_CONFIG") {
            let env_path = PathBuf::from(env_path_str);
            if exists(&env_path) {
                // Load from the environment variable path
                let env_content = read_string(env_path.to_str().unwrap())?;
                if let Ok(models) = serde_json::from_str::<Vec<ModelRaw>>(&env_content) {
                    return Ok(models);
                } else {
                    eprintln!("Warning: Could not parse models from RAGIT_MODEL_CONFIG, falling back to defaults");
                }
            } else {
                eprintln!(
                    "Warning: RAGIT_MODEL_CONFIG points to non-existent file: {}",
                    env_path.display()
                );
            }
        }

        // Check for ~/.config/ragit/models.json
        let home_dir = match std::env::var("HOME") {
            Ok(path) => path,
            Err(_) => {
                eprintln!("Warning: HOME environment variable not set, cannot check ~/.config/ragit/models.json");
                String::new()
            }
        };

        if !home_dir.is_empty() {
            let config_path = join_paths(
                &PathBuf::from(home_dir),
                &join_paths(
                    &PathBuf::from(".config"),
                    &join_paths(&PathBuf::from("ragit"), &PathBuf::from(MODEL_FILE_NAME))?,
                )?,
            )?;
            if exists(&config_path) {
                // Load from ~/.config/ragit/models.json
                let config_content = read_string(config_path.to_str().unwrap())?;
                if let Ok(models) = serde_json::from_str::<Vec<ModelRaw>>(&config_content) {
                    return Ok(models);
                } else {
                    eprintln!("Warning: Could not parse models from ~/.config/ragit/models.json, falling back to defaults");
                }
            }
        }

        // Fall back to default models
        Ok(ModelRaw::default_models())
    }

    // Remove API keys from models to ensure they're not stored in the local file
    fn remove_api_keys_from_models(&self, models: Vec<ModelRaw>) -> Vec<ModelRaw> {
        models
            .into_iter()
            .map(|model| {
                // First convert ModelRaw to Model
                if let Ok(mut model_obj) = Model::try_from(&model) {
                    // Create a new Model with api_key set to None
                    model_obj.api_key = None;
                    // Convert back to ModelRaw
                    ModelRaw::from(&model_obj)
                } else {
                    // If conversion fails, return the original model
                    // This shouldn't happen in practice
                    model
                }
            })
            .collect()
    }
}
