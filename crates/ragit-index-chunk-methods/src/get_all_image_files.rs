use crate::prelude::*;

pub fn get_all_image_files(index: &Index) -> Result<Vec<PathBuf>, ApiError> {
    let mut result = vec![];

    for internal in read_dir(&join3(&index.root_dir.to_string_lossy(), INDEX_DIR_NAME, IMAGE_DIR_NAME)?, false)? {
        if !is_dir(&internal) {
            continue;
        }

        for image_file in read_dir(&internal, false)? {
            if extension(&image_file).unwrap_or(None).unwrap_or(String::new()) == "png" {
                result.push(image_file.into());
            }
        }
    }

    // the result has to be deterministic
    result.sort();
    Ok(result)
}
