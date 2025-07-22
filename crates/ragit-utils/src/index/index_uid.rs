use crate::constant::{II_DIR_NAME, INDEX_DIR_NAME, INDEX_FILE_NAME};
use crate::error::Error;
use crate::index::index_struct::Index;
use crate::index::load_mode::LoadMode;
use crate::prelude::*;
use ragit_uid::{load_from_file, Uid, UidWriteMode};
use ragit_fs::{exists, file_name, join, join3, join4, normalize, parent, read_dir, read_string, remove_file, set_extension, try_create_dir, write_bytes, write_string, WriteMode};
use sha3::{Digest, Sha3_256};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

impl Index {
    pub fn calculate_and_save_uid(&mut self) -> Result<Uid, Error> {
        match self.uid {
            Some(uid) => Ok(uid),
            None => {
                let uid = self.calculate_uid(false  /* force */)?;
                self.uid = Some(uid);
                self.save_to_file(self.root_dir.join(INDEX_FILE_NAME))?;
                Ok(uid)
            },
        }
    }

    pub fn calculate_uid(&self, force: bool) -> Result<Uid, Error> {
        match self.uid {
            Some(uid) if !force => Ok(uid),
            _ => {
                let mut uids = vec![];

                for chunk_path in self.get_all_chunk_files()?.iter() {
                    let chunk = crate::chunk::load_from_file(chunk_path)?;
                    uids.push(chunk.uid);
                }

                for image_path in self.get_all_image_files()?.iter() {
                    let image_uid_prefix = file_name(parent(image_path.as_path())?.to_str().unwrap())?;
                    let image_uid_suffix = file_name(image_path.to_str().unwrap())?;
                    let uid = format!("{}{}", image_uid_prefix, image_uid_suffix).parse::<Uid>()?;
                    uids.push(uid);
                }

                for file_index_path in self.get_all_file_indexes()?.iter() {
                    for uid in load_from_file(&PathBuf::from(file_index_path.to_str().unwrap()))? {
                        uids.push(uid);
                    }
                }

                let mut result = Uid::new_knowledge_base(&uids);

                if let Some(summary_content) = &self.summary {
                    let summary_uid = Uid::new_summary(summary_content);
                    if summary_uid == result {
                        uids.push(summary_uid);
                        result = Uid::new_knowledge_base(&uids);
                    }
                }

                Ok(result)
            },
        }
    }

    pub fn reset_uid(
        &mut self,
        save_to_file: bool,
    ) -> Result<(), Error> {
        if self.uid.is_some() {
            self.uid = None;

            if save_to_file {
                self.save_to_file(self.root_dir.join(INDEX_FILE_NAME))?;
            }
        }

        Ok(())
    }
}