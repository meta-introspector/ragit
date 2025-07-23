use ragit_utils::constant::IMAGE_DIR_NAME;
use ragit_utils::error::Error;
use ragit_readers::ImageDescription;
use ragit_utils::ragit_path_utils::get_uid_path;
use ragit_fs::{read_bytes, read_string};
use ragit_uid::Uid;
use std::path::Path;

impl super::Index {
    pub fn get_image_bytes_by_uid(&self, uid: Uid) -> Result<Vec<u8>, Error> {
        Ok(read_bytes(
            get_uid_path(&self.root_dir, &Path::new(IMAGE_DIR_NAME), uid, Some("png"))?
                .to_str()
                .unwrap(),
        )?)
    }

    pub fn get_image_description_by_uid(&self, uid: Uid) -> Result<ImageDescription, Error> {
        let j = read_string(
            get_uid_path(&self.root_dir, &Path::new(IMAGE_DIR_NAME), uid, Some("json"))?
                .to_str()
                .unwrap(),
        )?;
        let v = serde_json::from_str::<ImageDescription>(&j)?;
        Ok(v)
    }
}
