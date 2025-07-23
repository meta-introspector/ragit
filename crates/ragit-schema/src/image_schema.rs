use ragit_types::{Uid, ImageSchema};
use anyhow::Error;
use ragit_utils::index::Index;

use ragit_utils::constant::IMAGE_DIR_NAME;
use ragit_fs::{file_size, read_bytes, read_string, set_extension};
use ragit_pdl::JsonType;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use ragit_core_utils::path::get_uid_path;

pub fn get_image_schema(index: &Index, uid: Uid, load_bytes: bool) -> Result<ImageSchema, Error> {
    let description_path = get_uid_path(
        &index.root_dir,
        IMAGE_DIR_NAME,
        uid,
        Some("json"),
    )?;
    let image_path = set_extension(description_path.to_string_lossy().as_ref(), "png")?;
    let description = read_string(description_path.to_string_lossy().as_ref())?;
    let description = serde_json::from_str::<Value>(&description)?;
    let bytes = if load_bytes {
        read_bytes(PathBuf::from(image_path.clone()).to_string_lossy().as_ref())?
    } else {
        vec![]
    };

    match description {
        Value::Object(obj) => match (obj.get("extracted_text"), obj.get("explanation")) {
            (Some(Value::String(extracted_text)), Some(Value::String(explanation))) => Ok(ImageSchema {
                uid,
                extracted_text: extracted_text.to_string(),
                explanation: explanation.to_string(),
                size: file_size(&image_path)?,
                bytes,
            }),
            _ => Err(Error::BrokenIndex(format!("`{}` has a wrong schema.", description_path.display()))),
        },
        _ => Err(Error::JsonTypeError {
            expected: JsonType::Object,
            got: (&description).into(),
        }),
    }
}