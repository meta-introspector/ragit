use serde::de::DeserializeOwned;
use crate::Error;
use ragit_fs::{exists, read_string, join};
use std::path::PathBuf;
use crate::config::partial_api_config::PartialApiConfig;
use crate::config::partial_build_config::PartialBuildConfig;
use crate::config::partial_query_config::PartialQueryConfig;

/// Attempts to load a config file from ~/.config/ragit/
pub fn load_config_from_home<T: DeserializeOwned>(
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

    let config_path = join(
        &home_dir,
        &join(
            ".config",
            &join("ragit", filename)?,
        )?,
    )?;

    if exists(&PathBuf::from(config_path.clone())) {
        // Load from ~/.config/ragit/filename
        let config_content = read_string(config_path.as_str())?;
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

pub fn load_api_config_from_home() -> Result<Option<PartialApiConfig>, Error> {
    load_config_from_home("api.json")
}

pub fn load_query_config_from_home() -> Result<Option<PartialQueryConfig>, Error> {
    load_config_from_home("query.json")
}

pub fn load_build_config_from_home() -> Result<Option<PartialBuildConfig>, Error> {
    load_config_from_home("build.json")
}
