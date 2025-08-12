use serde_json::{Value, to_string_pretty};
use anyhow::{Result, anyhow};
use std::fs::{self, File};
use std::io::{Write, BufWriter};
use std::path::{Path, PathBuf};
use rayon::prelude::*;

const MAX_CHUNK_SIZE_BYTES: usize = 10 * 1024 * 1024; // 10 MB
const PRIMES: &[usize] = &[2, 3, 5, 7, 11, 13, 17, 19];

fn split_value_recursively(
    value: &Value,
    output_dir: &Path,
    base_filename: &str,
    current_prime_idx: usize,
    file_counter: &mut usize,
) -> Result<()> {
    let current_size = to_string_pretty(value)?.len();

    if current_size <= MAX_CHUNK_SIZE_BYTES || current_prime_idx >= PRIMES.len() {
        // If within size limits or no more primes, save the value as a file
        let file_name = format!("{}_{}.json", base_filename, *file_counter);
        let output_path = output_dir.join(&file_name);
        *file_counter += 1;

        fs::create_dir_all(output_dir)?;
        let mut file = BufWriter::new(File::create(&output_path)?);
        file.write_all(to_string_pretty(value)?.as_bytes())?;
        file.flush()?;
        return Ok(());
    }

    let prime = PRIMES[current_prime_idx];
    let num_sub_chunks = prime;

    if let Some(arr) = value.as_array() {
        let chunk_size = (arr.len() + num_sub_chunks - 1) / num_sub_chunks;
        for i in 0..num_sub_chunks {
            let start = i * chunk_size;
            let end = (start + chunk_size).min(arr.len());
            if start >= end {
                continue;
            }
            let sub_array = Value::Array(arr[start..end].to_vec());
            split_value_recursively(
                &sub_array,
                output_dir,
                base_filename,
                current_prime_idx + 1,
                file_counter,
            )?;
        }
    } else if let Some(obj) = value.as_object() {
        // For objects, we can split by keys if there are many, or just recurse on values
        // For simplicity, let's just recurse on each value for now.
        // A more advanced strategy might involve grouping keys or splitting the object itself.
        let keys: Vec<String> = obj.keys().cloned().collect();
        let chunk_size = (keys.len() + num_sub_chunks - 1) / num_sub_chunks;

        for i in 0..num_sub_chunks {
            let start = i * chunk_size;
            let end = (start + chunk_size).min(keys.len());
            if start >= end {
                continue;
            }
            let mut sub_object = serde_json::Map::new();
            for key_name in &keys[start..end] {
                sub_object.insert(key_name.clone(), obj[key_name].clone());
            }
            split_value_recursively(
                &Value::Object(sub_object),
                output_dir,
                base_filename,
                current_prime_idx + 1,
                file_counter,
            )?;
        }
    } else {
        // If it's a primitive value and still too large (shouldn't happen if MAX_CHUNK_SIZE_BYTES is reasonable)
        // just save it.
        let file_name = format!("{}_{}.json", base_filename, *file_counter);
        let output_path = output_dir.join(&file_name);
        *file_counter += 1;

        fs::create_dir_all(output_dir)?;
        let mut file = BufWriter::new(File::create(&output_path)?);
        file.write_all(to_string_pretty(value)?.as_bytes())?;
        file.flush()?;
    }

    Ok(())
}

pub fn split_matrix_file(
    input_path: &Path,
    base_output_dir: &Path,
) -> Result<()> {
    let file_content = fs::read_to_string(input_path)?;
    let parsed_json: Value = serde_json::from_str(&file_content)?;

    let matrix_object = parsed_json.as_object().ok_or_else(|| anyhow!("Input JSON is not a top-level object."))?;

    if matrix_object.is_empty() {
        println!("Input file {} is empty, no splitting needed.", input_path.display());
        return Ok(());
    }

    println!("Extracting {} elements from {}...", matrix_object.len(), input_path.display());

    let elements_to_process: Vec<(&String, &Value)> = matrix_object.iter().collect();
    let mut file_counter_global = 0;

    elements_to_process.into_par_iter().try_for_each(|(key, value)| -> Result<()> {
        let mut current_dir = base_output_dir.to_path_buf();

        // First letter directory for the key
        if let Some(first_char) = key.chars().next() {
            current_dir = current_dir.join(first_char.to_lowercase().to_string());
        }

        fs::create_dir_all(&current_dir)?;

        // Recursively split the value associated with the key
        let mut file_counter_local = 0;
        split_value_recursively(
            value,
            &current_dir,
            key,
            0,
            &mut file_counter_local,
        )?;

        Ok(())
    })?;

    println!("Extraction complete.");
    Ok(())
}
