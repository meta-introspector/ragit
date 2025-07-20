use std::path::{Path, PathBuf};
use crate::error::Error;
use ragit_fs::{join, join3, normalize, into_abs_path};
use crate::constant::{II_DIR_NAME, INDEX_DIR_NAME};
use crate::uid::Uid;

/// Converts a `PathBuf` to an `&str`. Panics if the path is not valid UTF-8.
pub fn pathbuf_to_str(path: &PathBuf) -> &str {
    path.to_str().expect("Path is not valid UTF-8")
}

/// Converts an `&str` to a `PathBuf`.
pub fn str_to_pathbuf(s: &str) -> PathBuf {
    PathBuf::from(s)
}

/// Returns a displayable object for a `Path`.
pub fn path_to_display<'a>(path: &'a Path) -> impl std::fmt::Display + 'a {
    path.display()
}

/// Converts an `&str` to an `&Path`.
pub fn str_to_path_ref(s: &str) -> &Path {
    Path::new(s)
}

pub fn join_paths(path: &Path, child: &Path) -> Result<PathBuf, Error> {
    let joined = join(pathbuf_to_str(&path.to_path_buf()), pathbuf_to_str(&child.to_path_buf()))?;
    Ok(PathBuf::from(joined))
}

pub fn join3_paths(path1: &Path, path2: &Path, path3: &Path) -> Result<PathBuf, Error> {
    let joined = join3(pathbuf_to_str(&path1.to_path_buf()), pathbuf_to_str(&path2.to_path_buf()), pathbuf_to_str(&path3.to_path_buf()))?;
    Ok(PathBuf::from(joined))
}

pub fn get_rag_path(root_dir: &PathBuf, rel_path: &PathBuf) -> Result<PathBuf, Error> {
    let abs_root_dir_str = into_abs_path(pathbuf_to_str(root_dir))?;
    let abs_root_dir = str_to_pathbuf(normalize(&abs_root_dir_str)?.as_str());
    let joined_path = join_paths(abs_root_dir.as_path(), rel_path.as_path())?;
    Ok(str_to_pathbuf(normalize(pathbuf_to_str(&joined_path))?.as_str()))
}

pub(crate) fn get_uid_path(root_dir: &PathBuf, dir: &str, uid: Uid, ext: Option<&str>) -> Result<PathBuf, Error> {
    let uid_str = uid.to_string();
    let uid_prefix = &uid_str[0..2];
    let uid_suffix = &uid_str[2..];

    let path = join3_paths(
        root_dir,
        &str_to_pathbuf(dir),
        &str_to_pathbuf(uid_prefix),
    )?;

    let final_path = join_paths(
        path.as_path(),
        &str_to_pathbuf(uid_suffix),
    )?;

    if let Some(ext) = ext {
        Ok(final_path.with_extension(ext))
    } else {
        Ok(final_path)
    }
}

// root_dir/.ragit/ii/term_hash_prefix/term_hash_suffix
pub(crate) fn get_ii_path(root_dir: &PathBuf, term_hash: String) -> PathBuf {
    let ii_at = join3_paths(
        root_dir,
        &str_to_pathbuf(INDEX_DIR_NAME),
        &str_to_pathbuf(II_DIR_NAME),
    ).unwrap();
    let term_hash_prefix = term_hash.get(0..2).unwrap().to_string();
    let term_hash_suffix = term_hash.get(2..).unwrap().to_string();

    join3_paths(
        &ii_at,
        &str_to_pathbuf(&term_hash_prefix),
        &str_to_pathbuf(&term_hash_suffix),
    ).unwrap()
}

pub(crate) fn get_ii_path_str(root_dir: &str, term_hash: String) -> PathBuf {
    let ii_at = join3(
        root_dir,
        INDEX_DIR_NAME,
        II_DIR_NAME,
    ).unwrap();
    let term_hash_prefix = term_hash.get(0..2).unwrap().to_string();
    let term_hash_suffix = term_hash.get(2..).unwrap().to_string();

    join3(
        &ii_at,
        &term_hash_prefix,
        &term_hash_suffix,
    ).unwrap().into()
}

