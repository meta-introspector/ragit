use std::path::{Path, PathBuf};
use ragit_fs::{into_abs_path, join, join3, normalize, is_dir, is_symlink, read_dir};

pub fn get_relative_path(root_dir: &Path, file: &Path) -> Result<PathBuf, std::io::Error> {
    let file_abs = into_abs_path(file.to_str().unwrap())?;
    let root_abs = into_abs_path(root_dir.to_str().unwrap())?;
    let result = file_abs.strip_prefix(root_abs).map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "not in root"))?;
    Ok(result.to_path_buf())
}

pub fn join_paths(path: &Path, child: &Path) -> Result<PathBuf, std::io::Error> {
    let joined = join(&path.to_str().unwrap(), &child.to_str().unwrap())?;
    Ok(PathBuf::from(joined))
}

pub fn join3_paths(path1: &Path, path2: &Path, path3: &Path) -> Result<PathBuf, std::io::Error> {
    let joined = join3(&path1.to_str().unwrap(), &path2.to_str().unwrap(), &path3.to_str().unwrap())?;
    Ok(PathBuf::from(joined))
}

pub fn get_normalized_abs_pathbuf(path: &Path) -> Result<PathBuf, std::io::Error> {
    Ok(PathBuf::from(normalize(&into_abs_path(path.to_str().unwrap())?)?))
}
