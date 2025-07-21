use crate::prelude::*;
use std::path::PathBuf;
use ragit_fs;

pub fn find_root() -> Result<PathBuf, Error> {
    let current_dir = std::env::current_dir()?;
    let mut current_dir: PathBuf = current_dir.to_path_buf();

    loop {
        if ragit_fs::exists(&ragit_fs::join(&current_dir.to_string_lossy().into_owned(), &INDEX_DIR_NAME.to_string())?.into()) {
            return Ok(current_dir);
        }

        if let Some(parent_dir) = current_dir.parent() {
            current_dir = parent_dir.to_path_buf();
        } else {
            return Err(Error::IndexNotFound);
        }
    }
}