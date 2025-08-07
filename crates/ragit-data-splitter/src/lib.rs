use serde::{Deserialize, Serialize};
use serde_json::{Value, to_string_pretty};
use anyhow::{Result, anyhow};
use std::fs::{self, File};
use std::io::{Write, BufWriter};
use std::path::{Path, PathBuf};
use rayon::prelude::*;

pub mod matrix_splitter;

#[derive(Debug, Deserialize, Serialize)]
pub struct AugmentedTermsData {
    pub augmented_terms: Vec<Value>,
}

pub fn extract_and_save_elements(
    input_path: &Path,
    base_output_dir: &Path,
) -> Result<()> {
    let file_content = fs::read_to_string(input_path)?;
    let parsed_json: Value = serde_json::from_str(&file_content)?;

    let items_to_process: Vec<Value>;

    // Try to get an array directly, or find the largest array within an object
    if let Some(arr) = parsed_json.as_array() {
        items_to_process = arr.clone();
    } else if let Some(obj) = parsed_json.as_object() {
        let mut largest_array: Option<&Vec<Value>> = None;
        for (_key, value) in obj {
            if let Some(arr) = value.as_array() {
                if largest_array.is_none() || arr.len() > largest_array.unwrap().len() {
                    largest_array = Some(arr);
                }
            }
        }
        items_to_process = largest_array.cloned().unwrap_or_else(Vec::new);
    } else {
        return Err(anyhow!("Input JSON is neither an array nor an object containing an array."));
    }

    if items_to_process.is_empty() {
        println!("No elements to extract from {}.", input_path.display());
        return Ok(());
    }

    println!("Extracting {} elements from {}...", items_to_process.len(), input_path.display());

    // Use par_iter for parallel processing
    items_to_process.into_par_iter().enumerate().try_for_each(|(i, element_value)| -> Result<()> {
        let mut current_dir = base_output_dir.to_path_buf();

        // Attempt to get a meaningful name for directory creation, fallback to index
        let dir_name_prefix = if let Some(term_name) = element_value.get("term").and_then(|v| v.as_str()) {
            term_name.chars().next().map(|c| c.to_lowercase().to_string()).unwrap_or_else(|| i.to_string())
        } else if let Some(id) = element_value.get("id").and_then(|v| v.as_str()) {
            id.chars().next().map(|c| c.to_lowercase().to_string()).unwrap_or_else(|| i.to_string())
        } else {
            i.to_string()
        };

        current_dir = current_dir.join(dir_name_prefix);

        fs::create_dir_all(&current_dir)?;

        // Use a sequential number as the file name
        let file_name = format!("{}.json", i);
        let output_path = current_dir.join(&file_name);

        let element_json = to_string_pretty(&element_value)?;

        let mut file = BufWriter::new(File::create(&output_path)?);
        file.write_all(element_json.as_bytes())?;
        file.flush()?;

        Ok(())
    })?;

    Ok(())
}
