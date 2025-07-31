use ragit_types::pdl_error::PdlError as Error;
use ragit_types::pdl_types::{Message, Role};


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
