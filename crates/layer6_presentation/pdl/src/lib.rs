use serde::Serialize;
use serde_json::Value;
use crate::error::Error;
use crate::schema::Schema;
use crate::message::Message;
use crate::image::ImageType;
use crate::role::Role;
use crate::util::decode_base64;
use crate::util::JsonType;

mod error;
mod image;
mod message;
mod role;
pub mod schema;
mod util;

pub mod prelude;
pub use prelude::*;
pub mod validate_last_message_role;
pub mod parse_pdl_from_file;
pub mod parse_pdl;
pub mod escape_pdl_tokens;
pub mod unescape_pdl_tokens;
pub mod into_message_contents;
pub mod try_parse_inline_block;
pub mod try_get_pdl_token;


/// `parse_pdl` takes `tera::Context` as an input. If you're too lazy to
/// construct a `tera::Context`, you can use this function. It converts
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
        _ => Err(Error::JsonTypeError {
            expected: JsonType::Object,
            got: (&v).into(),
        }),
    }
}

#[derive(Clone, Debug)]
pub struct Pdl {
    pub schema: Option<Schema>,
    pub messages: Vec<Message>,
}

impl Pdl {
    pub fn validate(&self) -> Result<(), Error> {
        if self.messages.is_empty() {
            return Err(Error::InvalidPdl(String::from("A pdl file is empty.")));
        }

        let mut after_user = false;
        let mut after_assistant = false;

        for (index, Message { role, .. }) in self.messages.iter().enumerate() {
            match role {
                Role::User => {
                    if after_user {
                        return Err(Error::InvalidPdl(String::from(
                            "<|user|> appeared twice in a row.",
                        )));
                    }

                    after_user = true;
                    after_assistant = false;
                }
                Role::Assistant => {
                    if after_assistant {
                        return Err(Error::InvalidPdl(String::from(
                            "<|assistant|> appeared twice in a row.",
                        )));
                    }

                    after_user = false;
                    after_assistant = true;
                }
                Role::System => {
                    if index != 0 {
                        return Err(Error::InvalidPdl(String::from(
                            "<|system|> must appear at top.",
                        )));
                    }
                }
                Role::Reasoning => {} // TODO
            }
        }

        super::validate_last_message_role::validate_last_message_role(&self.messages)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ImageType, Message, MessageContent, Pdl, Role, decode_base64, parse_pdl,
        parse_pdl_from_file,
    };
    use ragit_fs::{WriteMode, join, temp_dir, write_string};

    // more thorough test suites are in `tests/`
    #[test]
    fn messages_from_file_test() {
        let tmp_path = join(&temp_dir().unwrap(), "test_messages.tera").unwrap();

        write_string(
            &tmp_path,
            "
<|system|>

You're a code helper.

<|user|>

Write me a sudoku-solver.


",
            WriteMode::CreateOrTruncate,
        )
        .unwrap();

        let Pdl { messages, schema } =
            parse_pdl_from_file(&tmp_path, &tera::Context::new(), true).unwrap();

        assert_eq!(
            messages,
            vec![
                Message {
                    role: Role::System,
                    content: vec![MessageContent::String(String::from(
                        "You're a code helper."
                    )),],
                },
                Message {
                    role: Role::User,
                    content: vec![MessageContent::String(String::from(
                        "Write me a sudoku-solver."
                    )),],
                },
            ],
        );
        assert_eq!(schema, None,);
    }

    #[test]
    fn media_content_test() {
        let Pdl { messages, schema } = parse_pdl(
            "
<|user|>

<|raw_media(png:HiMyNameIsBaehyunsol)|>
",
            &tera::Context::new(),
            ".", // there's no `<|media|>`
            true,
        )
        .unwrap();

        assert_eq!(
            messages,
            vec![Message {
                role: Role::User,
                content: vec![MessageContent::Image {
                    image_type: ImageType::Png.into(),
                    bytes: decode_base64("HiMyNameIsBaehyunsol").unwrap(),
                },],
            },],
        );
        assert_eq!(schema, None,);
    }
}
