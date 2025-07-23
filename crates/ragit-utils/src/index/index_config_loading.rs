use crate::api_config::{ApiConfig, PartialApiConfig};
use crate::error::Error;
use crate::path_utils::{join_paths, str_to_pathbuf};
use ragit_fs::{exists, read_string};

use crate::index::index_struct::Index;

impl Index {
    /// Attempts to load a config file from ~/.config/ragit/
    pub fn load_config_from_home<T: serde::de::DeserializeOwned>(
        filename: &str,
    ) -> Result<Option<T>, Error> {
        // Check for HOME environment variable
        let home_dir = match std::env::var("HOME") {
            Ok(path) => path,
            Err(_) => {
                eprintln!(
                    "Warning: HOME environment variable not set, cannot check ~/.config/ragit/{}",
                    filename
                );
                return Ok(None);
            }
        };

        let config_path = join_paths(
            &str_to_pathbuf(&home_dir),
            &join_paths(
                &str_to_pathbuf(".config"),
                &join_paths(&str_to_pathbuf("ragit"), &str_to_pathbuf(filename))?,
            )?,
        )?;

        if exists(&config_path.clone().into()) {
            // Load from ~/.config/ragit/filename
            let config_content = read_string(config_path.to_str().unwrap())?;
            match serde_json::from_str::<T>(&config_content) {
                Ok(config) => {
                    eprintln!(
                        "Info: Using configuration from ~/.config/ragit/{}",
                        filename
                    );
                    return Ok(Some(config));
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Could not parse {} from ~/.config/ragit/{}: {}",
                        filename, filename, e
                    );
                }
            }
        }

        Ok(None)
    }

    /// Attempts to load PartialApiConfig from ~/.config/ragit/api.json
    pub(crate) fn load_api_config_from_home(&self) -> Result<Option<PartialApiConfig>, Error> {
        Index::load_config_from_home("api.json")
    }

    /// Attempts to load PartialQueryConfig from ~/.config/ragit/query.json
    pub(crate) fn load_query_config_from_home(
        &self,
    ) -> Result<Option<crate::query::config::PartialQueryConfig>, Error> {
        Index::load_config_from_home("query.json")
    }

    /// Attempts to load PartialBuildConfig from ~/.config/ragit/build.json
    pub(crate) fn load_build_config_from_home(
        &self,
    ) -> Result<Option<crate::index::config::PartialBuildConfig>, Error> {
        Index::load_config_from_home("build.json")
    }

    /// Returns a default ApiConfig with a valid model.
    /// If ~/.config/ragit/api.json exists, values from there will override the defaults.
    /// If the default model doesn't exist in the loaded models,
    /// it selects the lowest-cost model instead.
    pub(crate) fn get_default_api_config(&self) -> Result<ApiConfig, Error> {
        // Start with default config
        let mut config = ApiConfig::default();

        // Try to load partial api config from home directory
        if let Ok(Some(home_config)) = self.load_api_config_from_home() {
            home_config.apply_to(&mut config);
        }

        // Check if the model exists in the loaded models
        let model_exists = ragit_api::get_model_by_name(&self.models, &config.model).is_ok();

        if !model_exists && !self.models.is_empty() {
            // Find the lowest-cost model
            if let Some(lowest_cost_model) = self.find_lowest_cost_model() {
                // Update the model in the config
                config.model = lowest_cost_model.name.clone();
                eprintln!("Warning: Model '{}' not found in models.json. Using lowest-cost model '{}' instead.", 
                         config.model, lowest_cost_model.name);
            }
        }

        Ok(config)
    }
}
