use ragit_fs::{into_abs_path, join, join3, normalize};
use std::io;
use std::path::{Path, PathBuf};

pub fn get_relative_path(root_dir: &Path, file: &Path) -> Result<PathBuf, io::Error> {
    let file_abs = PathBuf::from(into_abs_path(file.to_str().unwrap()).map_err(io::Error::from)?);
    let root_abs =
        PathBuf::from(into_abs_path(root_dir.to_str().unwrap()).map_err(io::Error::from)?);
    let result = file_abs
        .strip_prefix(&root_abs)
        .map(|p| p.to_path_buf()).map_err(|_| io::Error::other("not in root"))?;
    Ok(result)
}

pub fn join_paths(path: &Path, child: &Path) -> Result<PathBuf, io::Error> {
    let joined =
        join(path.to_str().unwrap(), child.to_str().unwrap()).map_err(io::Error::from)?;
    Ok(PathBuf::from(joined))
}

pub fn join3_paths(path1: &Path, path2: &Path, path3: &Path) -> Result<PathBuf, io::Error> {
    let joined = join3(
        path1.to_str().unwrap(),
        path2.to_str().unwrap(),
        path3.to_str().unwrap(),
    )
    .map_err(io::Error::from)?;
    Ok(PathBuf::from(joined))
}

pub fn get_normalized_abs_pathbuf(path: &Path) -> Result<PathBuf, io::Error> {
    Ok(PathBuf::from(normalize(
        &into_abs_path(path.to_str().unwrap()).map_err(io::Error::from)?,
    )?))
}
