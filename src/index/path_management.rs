use crate::constant::{CONFIG_DIR_NAME, API_CONFIG_FILE_NAME, BUILD_CONFIG_FILE_NAME, QUERY_CONFIG_FILE_NAME};
use crate::error::Error;
use ragit_fs::join;

pub type Path = String;

use crate::index::index_struct::Index;

impl Index {
    // every path in index.json are relative path to root_dir
    

    pub(crate) fn get_api_config_path(&self) -> Result<Path, Error> {
        Ok(crate::index::index_struct::Index::get_rag_path(
              &self.root_dir,
              &join(
                  CONFIG_DIR_NAME,
                  API_CONFIG_FILE_NAME,
              )?.into(),
          )?)
    }

    pub(crate) fn get_build_config_path(&self) -> Result<Path, Error> {
        Ok(crate::index::index_struct::Index::get_rag_path(
            &self.root_dir,
            &join(
                CONFIG_DIR_NAME,
                BUILD_CONFIG_FILE_NAME,
            )?,
        )?)
    }

    pub(crate) fn get_query_config_path(&self) -> Result<Path, Error> {
        Ok(crate::index::index_struct::Index::get_rag_path(
            &self.root_dir,
            &join(
                CONFIG_DIR_NAME,
                QUERY_CONFIG_FILE_NAME,
            )?,
        )?)
    }
}
