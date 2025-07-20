use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PROMPTS: HashMap<String, String> = {
        let mut result = HashMap::new();
        // TODO: Add actual prompt content or a mechanism to load them later
        result
    };
}