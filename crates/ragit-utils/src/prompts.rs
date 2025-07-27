use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::constant::PROMPT_DIR_NAME;

lazy_static! {
    pub static ref PROMPTS: HashMap<String, String> = {
        let mut result = HashMap::new();
        let prompts_dir = PathBuf::from(PROMPT_DIR_NAME);

        if prompts_dir.exists() && prompts_dir.is_dir() {
            for entry in fs::read_dir(&prompts_dir).expect("Failed to read prompts directory") {
                let entry = entry.expect("Failed to read directory entry");
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                        let content = fs::read_to_string(&path).expect(&format!("Failed to read prompt file: {:?}", path));
                        result.insert(file_name.to_string(), content);
                    }
                }
            }
        } else {
            eprintln!("Warning: Prompts directory not found at {:?}", prompts_dir);
        }
        result
    };
}
