use crate::constant::{CONFIG_DIR_NAME, API_CONFIG_FILE_NAME, BUILD_CONFIG_FILE_NAME, QUERY_CONFIG_FILE_NAME};
use crate::error::Error;
use crate::path_utils::{join_paths, str_to_pathbuf, get_rag_path};
use std::path::PathBuf;

pub type Path = PathBuf;

use crate::index::index_struct::Index;

impl Index {
    // every path in index.json are relative path to root_dir
    

    pub(crate) fn get_api_config_path(&self) -> Result<Path, Error> {
        Ok(get_rag_path(
              &self.root_dir,
              &join_paths(
                  &str_to_pathbuf(CONFIG_DIR_NAME),
                  &str_to_pathbuf(API_CONFIG_FILE_NAME),
              )?,
          )?)
    }

    pub(crate) fn get_build_config_path(&self) -> Result<Path, Error> {
        Ok(get_rag_path(
            &self.root_dir,
            &join_paths(
                &str_to_pathbuf(CONFIG_DIR_NAME),
                &str_to_pathbuf(BUILD_CONFIG_FILE_NAME),
            )?,
        )?)
    }

    pub(crate) fn get_query_config_path(&self) -> Result<Path, Error> {
        Ok(get_rag_path(
            &self.root_dir,
            &join_paths(
                &str_to_pathbuf(CONFIG_DIR_NAME),
                &str_to_pathbuf(QUERY_CONFIG_FILE_NAME),
            )?,
        )?)
    }
}
