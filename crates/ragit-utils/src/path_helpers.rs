use std::path::{Path, PathBuf};
use crate::error::Error;
use ragit_fs::{join, join3, normalize, into_abs_path};

pub fn pathbuf_to_str(path: &PathBuf) -> &str {
    path.to_str().unwrap_or_default()
}

pub fn str_to_pathbuf(s: &str) -> PathBuf {
    PathBuf::from(s)
}

pub fn join_paths(path: &Path, child: &Path) -> Result<PathBuf, Error> {
    let joined = join(path.to_str().unwrap(), child.to_str().unwrap())?;
    Ok(PathBuf::from(joined))
}

pub fn join3_paths(path1: &Path, path2: &Path, path3: &Path) -> Result<PathBuf, Error> {
    let joined = join3(path1.to_str().unwrap(), path2.to_str().unwrap(), path3.to_str().unwrap())?;
    Ok(PathBuf::from(joined))
}

pub fn get_rag_path(root_dir: &PathBuf, rel_path: &PathBuf) -> Result<PathBuf, Error> {
    let abs_root_dir = normalize(&into_abs_path(root_dir.to_str().unwrap())?)?;
    let joined_path = join_paths(&abs_root_dir, rel_path)?;
    Ok(normalize(&joined_path.to_str().unwrap())?)
}
