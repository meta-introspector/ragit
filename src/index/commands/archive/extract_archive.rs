use std::collections::HashMap;

use ragit_fs::{exists, join, remove_dir_all};

use crate::error::Error;
use crate::index::index_struct::Index;

use super::{block_type::BlockType, request::Request};

impl Index {
    pub fn extract_archive(
        root_dir: &str,
        archives: Vec<String>,
        workers: usize,
        force: bool,
        quiet: bool,
    ) -> Result<HashMap<BlockType, usize>, Error> {
        if exists(root_dir) {
            if force {
                if exists(&join(root_dir, crate::constant::INDEX_DIR_NAME)?) {
                    remove_dir_all(&join(root_dir, crate::constant::INDEX_DIR_NAME)?)?;
                }
            } else {
                return Err(Error::CannotExtractArchive(format!(
                    "`{root_dir}` already exists"
                )));
            }
        }

        let workers = super::init_workers::init_workers(workers, root_dir);

        match Index::extract_archive_worker(root_dir, archives, &workers, quiet) {
            Ok(result) => Ok(result),
            Err(e) => {
                for worker in workers.iter() {
                    let _ = worker.tx.send(Request::Kill);
                }

                if exists(root_dir) {
                    remove_dir_all(root_dir)?;
                }

                Err(e)
            }
        }
    }
}
