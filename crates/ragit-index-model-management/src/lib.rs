use ragit_index::Index;
use ragit_types::ApiError;
use ragit_fs::{exists, read_string, write_string, WriteMode};
use ragit_utils::ragit_path_utils::{get_rag_path, join_paths};
use ragit_utils::constant::MODEL_FILE_NAME;
use ragit_model::{Model, ModelRaw};
use anyhow::Result;
use std::path::PathBuf;

pub fn load_or_init_models(index: &mut Index) -> Result<(), ApiError> {
    let models_at = get_rag_path(&index.root_dir, &PathBuf::from(MODEL_FILE_NAME))
        .map_err(|e| ApiError::from(anyhow::Error::new(e)))?;

    if exists(&models_at) {
        let j = read_string(models_at.to_str().unwrap())?;
        let models = serde_json::from_str::<Vec<ModelRaw>>(&j)?;
        let mut result = Vec::with_capacity(models.len());

        for model in models.iter() {
            result.push(Model::try_from(model)?);
        }

        index.models = result;
    } else {
        index.models = Model::default_models();
        save_models(index)?;
    }

    Ok(())
}

pub fn save_models(index: &Index) -> Result<(), ApiError> {
    let models_at = get_rag_path(&index.root_dir, &PathBuf::from(MODEL_FILE_NAME))
        .map_err(|e| ApiError::from(anyhow::Error::new(e)))?;
    let models: Vec<ModelRaw> = index.models.iter().map(|model| model.into()).collect();
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
        &get_rag_path(&PathBuf::from("ragit"), &PathBuf::from(ragit_utils::constant::CONFIG_DIR_NAME))
            .map_err(|e| ApiError::from(anyhow::Error::new(e)))?,
        &PathBuf::from("models.json"),
    ).map_err(|e| ApiError::from(anyhow::Error::new(e)))?.to_str().unwrap()) {
        if let Ok(models) = serde_json::from_str::<Vec<ModelRaw>>(&env_content) {
            result.extend(models);
        }
    }

    if let Ok(config_content) = read_string(&join_paths(
        &get_rag_path(&PathBuf::from("ragit"), &PathBuf::from(ragit_utils::constant::CONFIG_DIR_NAME))
            .map_err(|e| ApiError::from(anyhow::Error::new(e)))?,
        &PathBuf::from("models.json"),
    ).map_err(|e| ApiError::from(anyhow::Error::new(e)))?.to_str().unwrap()) {
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

pub fn remove_api_keys_from_models(models: Vec<ModelRaw>) -> Vec<ModelRaw> {
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
