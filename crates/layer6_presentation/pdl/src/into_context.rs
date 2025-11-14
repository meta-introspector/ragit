use serde::Serialize;
use serde::ser::Error as SerdeError;
use serde_json::Value;
use ragit_types::pdl_error::PdlError as Error;

/// `parse_pdl` takes `tera::Context` as an input. If you're too lazy to
/// construct a `tera::Context`, you can be use this function. It converts
/// a rust struct into a json object.
pub fn into_context<T: Serialize>(v: &T) -> Result<tera::Context, Error> {
    let v = serde_json::to_value(v)?;
    let mut result = tera::Context::new();

    match v {
        Value::Object(object) => {
            for (k, v) in object.iter() {
                result.insert(k, v);
            }

            Ok(result)
        }
        _ => Err(Error::Json(serde_json::Error::custom(format!("Expected object, got: {:?}", v)))),
    }
}
