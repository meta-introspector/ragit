use serde::{Deserialize, Serialize};

use crate::prelude::{Message, Error};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TestModel {
    pub dummy_response: String,
}

impl TestModel {
    pub fn get_dummy_response(&self, _messages: &Vec<Message>) -> Result<String, Error> {
        Ok(self.dummy_response.clone())
    }
}
