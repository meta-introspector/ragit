use crate::constant::{II_DIR_NAME, INDEX_DIR_NAME};
use crate::error::Error;
//use ragit_core_utils::path::{path_to_display, str_to_path_ref};
use ragit_fs::{into_abs_path, join, join3, normalize};
use ragit_uid::Uid;
use std::path::{Path, PathBuf};

pub fn join_paths(path: &Path, child: &Path) -> Result<PathBuf, Error> {
    let joined = join(path.to_str().unwrap(), child.to_str().unwrap())?;
    Ok(PathBuf::from(joined))
}

pub fn join3_paths(path1: &Path, path2: &Path, path3: &Path) -> Result<PathBuf, Error> {
    let joined = join3(
        path1.to_str().unwrap(),
        path2.to_str().unwrap(),
        path3.to_str().unwrap(),
    )?;
    Ok(PathBuf::from(joined))
}

pub fn get_rag_path(root_dir: &PathBuf, rel_path: &PathBuf) -> Result<PathBuf, Error> {
    let abs_root_dir_str = into_abs_path(root_dir.to_str().unwrap())?;
    let abs_root_dir = PathBuf::from(normalize(&abs_root_dir_str)?.as_str());
    let joined_path = join_paths(abs_root_dir.as_path(), rel_path.as_path())?;
    Ok(PathBuf::from(
        normalize(joined_path.to_str().unwrap())?.as_str(),
    ))
}

pub fn get_uid_path(
    root_dir: &PathBuf,
    dir: &Path,
    uid: Uid,
    ext: Option<&str>,
) -> Result<PathBuf, Error> {
    let uid_str = uid.to_string();
    let uid_prefix = &uid_str[0..2];
    let uid_suffix = &uid_str[2..];

    let path = join3_paths(root_dir, dir, &PathBuf::from(uid_prefix))?;

    let final_path = join_paths(path.as_path(), &PathBuf::from(uid_suffix))?;

    if let Some(ext) = ext {
        Ok(final_path.with_extension(ext))
    } else {
        Ok(final_path)
    }
}

pub fn get_ii_path(root_dir: &PathBuf, term_hash: String) -> PathBuf {
    let ii_at = join3_paths(
        root_dir,
        &PathBuf::from(INDEX_DIR_NAME),
        &PathBuf::from(II_DIR_NAME),
    )
    .unwrap();
    let term_hash_prefix = term_hash.get(0..2).unwrap().to_string();
    let term_hash_suffix = term_hash.get(2..).unwrap().to_string();

    join3_paths(
        &ii_at,
        &PathBuf::from(&term_hash_prefix),
        &PathBuf::from(&term_hash_suffix),
    )
    .unwrap()
}

pub fn get_normalized_abs_pathbuf(path: &PathBuf) -> Result<PathBuf, Error> {
    Ok(PathBuf::from(normalize(&into_abs_path(
        path.to_str().unwrap(),
    )?)?))
}
