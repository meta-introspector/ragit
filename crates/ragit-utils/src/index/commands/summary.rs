use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub enum SummaryMode { Short, Long, }

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct Summary(pub String);

impl Deref for Summary {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Summary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub async fn summary_command(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    println!("summary_command is not yet implemented");
    Ok(())
}
