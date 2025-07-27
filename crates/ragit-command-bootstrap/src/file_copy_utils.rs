use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};

pub async fn copy_files_to_temp_dir(
    actual_root_dir: &PathBuf,
    temp_dir: &PathBuf,
    original_files_to_add: Vec<String>,
    verbose: bool,
) -> Result<Vec<PathBuf>> {
    let mut temp_files_to_add = Vec::new();
    for original_file_path_str in original_files_to_add {
        let original_file_path = PathBuf::from(original_file_path_str);
        if verbose {
            println!("bootstrap_index_self: going to add {}", original_file_path.display());
            println!("bootstrap_index_self: original_file_path: {:?}", original_file_path);
            println!("bootstrap_index_self: actual_root_dir: {:?}", actual_root_dir);
        }
        let relative_path = original_file_path.strip_prefix(actual_root_dir.as_path())
            .map_err(|e| anyhow::anyhow!("Failed to strip prefix from {:?}: {}", original_file_path.display(), e))?;
        if verbose {
            println!("bootstrap_index_self: relative_path (after strip_prefix): {:?}", relative_path);
        }

        let final_relative_path = match relative_path.strip_prefix(Path::new("/")) {
            Ok(stripped) => stripped,
            Err(_) => relative_path,
        };

        if verbose {
            println!("bootstrap_index_self: relative_path (adjusted): {:?}", final_relative_path);
            println!("bootstrap_index_self: temp_dir: {:?}", temp_dir);
        }
        let temp_file_path = temp_dir.join(final_relative_path);
        if verbose {
            println!("bootstrap_index_self: temp_file_path: {:?}", temp_file_path);
        }
	
        let parent_dir = temp_file_path.parent().unwrap();
        fs::create_dir_all(parent_dir)?;
        if verbose {
            println!("bootstrap_index_self: Created parent directory: {:?}", parent_dir);
            println!("bootstrap_index_self: Checking if original_file_path exists: {:?}", original_file_path.exists());
            println!("bootstrap_index_self: Checking if original_file_path is readable: {:?}", fs::metadata(&original_file_path).map(|m| !m.permissions().readonly()).unwrap_or(false));
            println!("bootstrap_index_self: Checking if parent_dir is writable: {:?}", fs::metadata(parent_dir).map(|m| !m.permissions().readonly()).unwrap_or(false));
        }
        fs::copy(&original_file_path, &temp_file_path)
            .context(format!("Failed to copy file from {:?} to {:?}", original_file_path, temp_file_path))?;
        if verbose {
            println!("bootstrap_index_self: Copied file {:?} to {:?}", original_file_path, temp_file_path);
        }
        temp_files_to_add.push(temp_file_path);
    }
    Ok(temp_files_to_add)
}