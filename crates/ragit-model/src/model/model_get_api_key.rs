use super::model_struct::Model;
use ragit_types::ApiError as Error;

impl Model {
    pub fn get_api_key(&self) -> Result<String, Error> {
        // Collect all available API keys
        let mut available_keys = Vec::new();

        // 1. Direct api_key (backward compatibility)
        if let Some(key) = &self.api_key {
            available_keys.push(key.clone());
        }

        // 2. Multiple api_keys
        if let Some(keys) = &self.api_keys {
            available_keys.extend(keys.clone());
        }

        // 3. Environment variable (backward compatibility)
        if let Some(var) = &self.api_env_var {
            if let Ok(key) = std::env::var(var) {
                available_keys.push(key);
            }
        }

        // 4. Multiple environment variables
        if let Some(vars) = &self.api_env_vars {
            for var in vars {
                if let Ok(key) = std::env::var(var) {
                    available_keys.push(key);
                }
            }
        }

        // 5. External model files (existing logic)
        if let Some(key) = self.find_api_key_in_external_files()? {
            available_keys.push(key);
        }

        if available_keys.is_empty() {
            return if self.api_key.is_some()
                || self.api_env_var.is_some()
                || self.api_keys.is_some()
                || self.api_env_vars.is_some()
            {
                Err(Error::ApiKeyNotFound {
                    env_var: self.api_env_var.clone(),
                })
            } else {
                Ok(String::new()) // No key required
            };
        }

        // Round-robin selection
        let index = self.current_key_index % available_keys.len();
        Ok(available_keys[index].clone())
    }
}
