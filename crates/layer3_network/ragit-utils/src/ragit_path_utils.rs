use crate::constant::{II_DIR_NAME, INDEX_DIR_NAME};
use crate::error::Error;
use ragit_fs::{into_abs_path, join, join3, normalize};
use ragit_types::uid::Uid;
use std::path::{Path, PathBuf};

pub fn get_rag_path(root_dir: &PathBuf, rel_path: &PathBuf) -> Result<PathBuf, Error> {
    let abs_root_dir_str = into_abs_path(root_dir.to_str().unwrap())?;
    let abs_root_dir = PathBuf::from(normalize(&abs_root_dir_str)?.as_str());
    let joined_path = join(abs_root_dir.to_str().unwrap(), rel_path.to_str().unwrap())?;
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

    let path = join3(root_dir.to_str().unwrap(), dir.to_str().unwrap(), uid_prefix)?;

    let final_path = join(path.as_str(), uid_suffix)?;

    if let Some(ext) = ext {
        Ok(PathBuf::from(final_path).with_extension(ext))
    } else {
        Ok(PathBuf::from(final_path))
    }
}

pub fn get_ii_path(root_dir: &PathBuf, term_hash: String) -> PathBuf {
    let ii_at = join3(
        root_dir.to_str().unwrap(),
        INDEX_DIR_NAME,
        II_DIR_NAME,
    )
    .unwrap();
    let term_hash_prefix = term_hash.get(0..2).unwrap().to_string();
    let term_hash_suffix = term_hash.get(2..).unwrap().to_string();

    PathBuf::from(join3(
        ii_at.as_str(),
        &term_hash_prefix,
        &term_hash_suffix,
    )
    .unwrap())
}

pub fn get_normalized_abs_pathbuf(path: &PathBuf) -> Result<PathBuf, Error> {
    Ok(PathBuf::from(normalize(&into_abs_path(
        path.to_str().unwrap(),
    )?)?))
}
