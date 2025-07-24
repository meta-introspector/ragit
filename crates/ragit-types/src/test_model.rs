use crate::ApiError as Error;
use crate::pdl_types::Message;
use std::io::{Read, Write, stdin, stdout};

#[derive(Clone, Debug, PartialEq)]
pub enum TestModel {
    Dummy, // it always returns `"dummy"`
    Stdin,
    Error, // it always raises an error
}

impl TestModel {
    pub fn get_dummy_response(&self, messages: &[Message]) -> Result<String, Error> {
        match self {
            TestModel::Dummy => Ok(String::from("dummy")),
            TestModel::Stdin => {
                for message in messages.iter() {
                    println!(
                        "<|{:?}|>\n\n{}\n\n",
                        message.role,
                        message
                            .content
                            .iter()
                            .map(|c| c.to_string())
                            .collect::<Vec<String>>()
                            .join(""),
                    );
                }

                print!("<|Assistant|>\n\n>>> ");
                stdout().flush()?;

                let mut s = String::new();
                stdin().read_to_string(&mut s)?;
                Ok(s)
            }
            TestModel::Error => Err(Error::TestModel),
        }
    }
}
