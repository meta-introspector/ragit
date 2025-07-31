use super::{Message, Role};
use crate::error::Error;

pub fn validate_last_message_role(messages: &[Message]) -> Result<(), Error> {
    if let Some(Message {
        role: Role::Assistant,
        ..
    }) = messages.last()
    {
        return Err(Error::InvalidPdl(String::from(
            "A pdl file ends with <|assistant|>.",
        )));
    }
    Ok(())
}
