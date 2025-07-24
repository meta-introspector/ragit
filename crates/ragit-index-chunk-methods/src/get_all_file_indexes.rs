use crate::prelude::*;

pub fn get_all_file_indexes(index: &Index) -> Result<Vec<PathBuf>, ApiError> {
    let mut result = vec![];

    for internal in read_dir(&join3(&index.root_dir.to_string_lossy(), INDEX_DIR_NAME, FILE_INDEX_DIR_NAME)?, false)? {
        if !is_dir(&internal) {
            continue;
        }

        for file_index in read_dir(&internal, false)? {
            result.push(file_index.into());
        }
    }

    // the result has to be deterministic
    result.sort();
    Ok(result)
}
