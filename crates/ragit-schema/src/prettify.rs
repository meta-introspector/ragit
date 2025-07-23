use ragit_types::{FileSchema, ImageSchema};
use anyhow::Error;
use serde_json::Value;

pub trait Prettify {
    fn prettify(&self) -> Result<Value, Error>;
}

impl Prettify for FileSchema {
    fn prettify(&self) -> Result<Value, Error> {
        let mut result = serde_json::to_value(self)?;

        if self.is_processed {
            if let Value::Object(obj) = &mut result {
                match obj.get_mut("uid") {
                    Some(uid) => { *uid = prettify_uid(uid) },
                    None => {},
                }

                match obj.get_mut("last_updated") {
                    Some(timestamp) => { *timestamp = prettify_timestamp(timestamp); },
                    None => {},
                }
            }
        }

        else {
            if let Value::Object(obj) = &mut result {
                for key in obj.keys().map(|k| k.to_string()).collect::<Vec<_>>() {
                    if key != "path" && key != "is_processed" {
                        obj.remove(&key);
                    }
                }
            }
        }

        Ok(result)
    }
}

impl Prettify for ImageSchema {
    fn prettify(&self) -> Result<Value, Error> {
        let mut result = serde_json::to_value(self)?;

        if let Value::Object(obj) = &mut result {
            match obj.get_mut("uid") {
                Some(uid) => { *uid = prettify_uid(uid); },
                None => {},
            }

            obj.remove("bytes");
        }

        Ok(result)
    }
}

// Placeholder functions
fn prettify_uid(uid: &mut Value) -> Value {
    uid.clone()
}

fn prettify_timestamp(timestamp: &mut Value) -> Value {
    timestamp.clone()
}
