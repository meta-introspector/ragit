use crate::prelude::*;

pub fn get_image_description_by_uid(index: &Index, uid: Uid) -> Result<ImageSchema, ApiError> {
    let j = read_string(&get_uid_path(&index.root_dir, Path::new(IMAGE_DIR_NAME), uid, Some("json"))?.to_string_lossy().into_owned())?;
    let v = serde_json::from_str::<ImageSchema>(&j)?;
    Ok(v)
}
