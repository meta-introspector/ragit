
use crate::prelude::*;
use ragit_fs::{file_size, read_bytes, read_string, set_extension};
use ragit_pdl::JsonType;
use ragit_utils::constant::IMAGE_DIR_NAME;
use serde_json::Value;
use std::path::{Path, PathBuf};

use ragit_utils::path_utils::get_uid_path;

impl Index {
    pub fn get_image_schema(&self, uid: Uid, load_bytes: bool) -> Result<ImageSchema, Error> {
        let description_path = get_uid_path(&self.root_dir, Path::new(IMAGE_DIR_NAME), uid, Some("json"))?;
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
                (Some(Value::String(extracted_text)), Some(Value::String(explanation))) => {
                    Ok(ImageSchema {
                        uid,
                        extracted_text: extracted_text.to_string(),
                        explanation: explanation.to_string(),
                        size: file_size(&image_path)?,
                        bytes,
                    })
                }
                _ => Err(Error::BrokenIndex(format!(
                    "`{}` has a wrong schema.",
                    description_path.display()
                ))),
            },
            _ => Err(Error::JsonTypeError {
                expected: JsonType::Object,
                got: (&description).into(),
            }),
        }
    }
}
