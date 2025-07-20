use std::path::PathBuf;
use crate::prelude::*;
use ragit_fs::{read_string, write_bytes, WriteMode, normalize, into_abs_path};

use super::BuildConfig;
use crate::index::{index_struct::Index, load_mode::LoadMode};

impl Index {
    pub fn load(
    root_dir: PathBuf,
    load_mode: LoadMode,
) -> Result<Self, Error> {
        let mut result = Index::load_minimum(root_dir)?;

        if load_mode == LoadMode::Minimum {
            return Ok(result);
        }

        result.build_config = serde_json::from_str::<BuildConfig>(
            &read_string(&result.get_build_config_path()?)?,
        )?;
        result.query_config = serde_json::from_str::<QueryConfig>(
            &read_string(&result.get_query_config_path()?)?,
        )?;
        result.api_config = serde_json::from_str::<ApiConfig>(
            &read_string(&result.get_api_config_path()?)?,
        )?;
        
        // Load models before initializing API config to ensure we can validate the model
        result.load_or_init_prompts()?;
        result.load_or_init_models()?;
        
        // Check if the model in api_config exists in the loaded models
        let model_exists = ragit_api::get_model_by_name(&result.models, &result.api_config.model).is_ok();

        if !model_exists && !result.models.is_empty() {
            // Find the lowest-cost model and update api_config
            if let Some(lowest_cost_model) = result.find_lowest_cost_model() {
                eprintln!(
                    "Warning: Model '{}' not found in models.json. Using lowest-cost model '{}' instead.", 
                    result.api_config.model,
                    lowest_cost_model.name,
                );

                // Update the model in the config
                result.api_config.model = lowest_cost_model.name.clone();

                // Save the updated config
                write_bytes(
                    &result.get_api_config_path()?,
                    &serde_json::to_vec_pretty(&result.api_config)?,
                    WriteMode::Atomic,
                )?;
            }
        }

        match load_mode {
            LoadMode::QuickCheck if result.curr_processing_file.is_some() => {
                result.recover()?;
                Ok(result)
            },
            LoadMode::Check if result.curr_processing_file.is_some() || result.check().is_err() => {
                result.recover()?;
                Ok(result)
            },
            _ => Ok(result),
        }
    }

    /// It only loads `index.json`. No config files, no prompt files, and it doesn't care whether chunk files are broken or not.
    /// It's for `rag check --recover`: it only loads minimum data and the recover function will load or fix the others.
    pub fn load_minimum(root_dir: PathBuf) -> Result<Self, Error> {
        let root_dir = normalize(&into_abs_path(&root_dir)?)?;
        let index_json = read_string(&Index::get_rag_path(
            &root_dir,
            &INDEX_FILE_NAME.to_string(),
        )?)?;

        let mut result = serde_json::from_str::<Index>(&index_json)?;
        result.root_dir = root_dir.into();

        let version_info = crate::index::commands::version::VersionInfo {
            version: result.ragit_version.clone(),
            compatible: true, // Assume compatible for now, actual compatibility check might be more complex
        };

        if let Some(warn) = crate::index::commands::version::get_compatibility_warning(&version_info) {
            eprintln!("Warning: {warn}");
        }

        Ok(result)
    }
}
