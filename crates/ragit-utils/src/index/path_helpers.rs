use crate::constant::{INDEX_DIR_NAME, II_DIR_NAME};
use crate::error::Error;
use crate::prelude::*;
use ragit_fs::{join, join3, normalize, set_extension};

use std::path::PathBuf;

use crate::index::index_struct::Index;

impl Index {
    // every path in index.json are relative path to root_dir
    pub(crate) fn get_rag_path(root_dir: &PathBuf, rel_path: &PathBuf) -> Result<PathBuf, Error> {
        Ok(normalize(
            &join3(
                root_dir,
                &INDEX_DIR_NAME.to_string(),
                rel_path,
            )?,
        )?)
    }

    pub(crate) fn get_data_path(root_dir: &PathBuf, rel_path: &PathBuf) -> Result<PathBuf, Error> {
        Ok(normalize(
            &join(
                root_dir.to_str().unwrap(),
                rel_path.to_str().unwrap(),
            )?,
        )?.into())
    }

    /// `{root_dir}/.ragit/{dir}/uid_prefix/uid_suffix(.{ext})?`
    pub(crate) fn get_uid_path(root_dir: &str, dir: &str, uid: Uid, ext: Option<&str>) -> Result<PathBuf, Error> {
        let dir = join3(
            root_dir,
            INDEX_DIR_NAME,
            dir,
        )?;
        let uid_prefix = uid.get_prefix();
        let uid_suffix = uid.get_suffix();

        let mut result = join3(
            &dir,
            &uid_prefix,
            &uid_suffix,
        )?;

        if let Some(ext) = ext {
            result = set_extension(&result, ext)?;
        }

        Ok(result)
    }

    // root_dir/.ragit/ii/term_hash_prefix/term_hash_suffix
    pub(crate) fn get_ii_path(root_dir: &PathBuf, term_hash: String) -> PathBuf {
        let ii_at = join3(
            root_dir,
            &INDEX_DIR_NAME,
            &II_DIR_NAME,
        ).unwrap();
        let term_hash_prefix = term_hash.get(0..2).unwrap().to_string();
        let term_hash_suffix = term_hash.get(2..).unwrap().to_string();

        join3(
            &ii_at,
            &term_hash_prefix,
            &term_hash_suffix,
        ).unwrap()
    }

    pub(crate) fn get_ii_path_str(root_dir: &str, term_hash: String) -> PathBuf {
        let ii_at = join3(
            root_dir,
            &INDEX_DIR_NAME,
            &II_DIR_NAME,
        ).unwrap();
        let term_hash_prefix = term_hash.get(0..2).unwrap().to_string();
        let term_hash_suffix = term_hash.get(2..).unwrap().to_string();

        join3(
            &ii_at,
            &term_hash_prefix,
            &term_hash_suffix,
        ).unwrap()
    }

    pub(crate) fn get_file_index_path(root_dir: &str, file_uid: Uid) -> Result<PathBuf, Error> {
        let file_index_dir = join3(
            root_dir,
            INDEX_DIR_NAME,
            crate::constant::FILE_INDEX_DIR_NAME,
        )?;
        let uid_prefix = file_uid.get_prefix();
        let uid_suffix = file_uid.get_suffix();

        let result = join3(
            &file_index_dir,
            &uid_prefix,
            &uid_suffix,
        )?;

        Ok(result)
    }
}
