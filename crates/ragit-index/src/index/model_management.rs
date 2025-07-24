use crate::prelude::*;

impl Index {
    pub fn load_or_init_models(&mut self) -> Result<(), ApiError> {
        let models_at = get_rag_path(&self.root_dir, MODEL_FILE_NAME)?;

        if exists(&models_at) {
            let j = read_string(&models_at)?;
            let models = serde_json::from_str::<Vec<ModelRaw>>(&j)?;
            let mut result = Vec::with_capacity(models.len());

            for model in models.iter() {
                result.push(Model::try_from(model)?);
            }

            self.models = result;
        } else {
            self.models = Model::default_models();
            self.save_models()?;
        }

        Ok(())
    }

    pub fn save_models(&self) -> Result<(), ApiError> {
        let models_at = get_rag_path(&self.root_dir, MODEL_FILE_NAME)?;
        let models: Vec<ModelRaw> = self.models.iter().map(|model| model.into()).collect();
        write_string(
            &models_at.to_str().unwrap(),
            &serde_json::to_string_pretty(&models)?,
            WriteMode::CreateOrTruncate,
        )?;

        Ok(())
    }

    pub fn get_initial_models() -> Result<Vec<ModelRaw>, ApiError> {
        let mut result = vec![];

        if let Ok(env_content) = read_string(&join_paths(
            &get_rag_path(&PathBuf::from("ragit"), CONFIG_DIR_NAME)?,
            "models.json",
        )?) {
            if let Ok(models) = serde_json::from_str::<Vec<ModelRaw>>(&env_content) {
                result.extend(models);
            }
        }

        if let Ok(config_content) = read_string(&join_paths(
            &get_rag_path(&PathBuf::from("ragit"), CONFIG_DIR_NAME)?,
            "models.json",
        )?) {
            if let Ok(models) = serde_json::from_str::<Vec<ModelRaw>>(&config_content) {
                result.extend(models);
            }
        }

        if result.is_empty() {
            Ok(ModelRaw::default_models())
        } else {
            Ok(result)
        }
    }

    fn remove_api_keys_from_models(&self, models: Vec<ModelRaw>) -> Vec<ModelRaw> {
        models
            .into_iter()
            .map(|model| {
                if let Ok(mut model_obj) = Model::try_from(&model) {
                    model_obj.api_key = None;
                    model_obj.api_env_vars = None;
                    ModelRaw::from(&model_obj)
                } else {
                    model
                }
            })
            .collect()
    }
}