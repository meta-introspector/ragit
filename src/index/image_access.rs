use crate::constant::IMAGE_DIR_NAME;
use crate::error::Error;
use crate::ImageDescription;
use crate::prelude::*;
use ragit_fs::{read_bytes, read_string};

use crate::index::index_struct::Index;

impl Index {
    pub fn get_image_bytes_by_uid(&self, uid: Uid) -> Result<Vec<u8>, Error> {
        Ok(read_bytes(&Index::get_uid_path(
            &self.root_dir,
            IMAGE_DIR_NAME,
            uid,
            Some("png"),
        )?)?)
    }

    pub fn get_image_description_by_uid(&self, uid: Uid) -> Result<ImageDescription, Error> {
        let j = read_string(Index::get_uid_path(
            self.root_dir.to_str().unwrap(),
            IMAGE_DIR_NAME,
            uid,
            Some("json"),
        )?)?;
        let v = serde_json::from_str::<ImageDescription>(&j)?;
        Ok(v)
    }
}
