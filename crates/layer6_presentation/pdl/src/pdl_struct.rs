use ragit_types::pdl_error::PdlError as Error;
use ragit_types::pdl_types::{Message, Role};
use ragit_types::schema::Schema;


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
