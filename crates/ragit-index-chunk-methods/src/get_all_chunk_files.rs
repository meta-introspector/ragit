use crate::prelude::*;

pub fn get_all_chunk_files(index: &Index) -> Result<Vec<PathBuf>, ApiError> {
    let mut result = vec![];

    for internal in read_dir(&join3(&index.root_dir.to_string_lossy(), INDEX_DIR_NAME, CHUNK_DIR_NAME)?, false)? {
        if !is_dir(&internal) {
            continue;
        }

        for chunk_file in read_dir(&internal, false)? {
            if extension(&chunk_file).unwrap_or(None).unwrap_or(String::new()) == "chunk" {
                result.push(chunk_file.into());
            }
        }
    }

    // the result has to be deterministic
    result.sort();
    Ok(result)
}
