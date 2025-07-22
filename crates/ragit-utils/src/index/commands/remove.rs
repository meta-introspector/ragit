use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoveResult {
    pub success: usize,
    pub errors: usize,
}

pub async fn remove_command(args: Vec<String>, pre_args: ragit_args::ParsedArgs) -> Result<(), Error> {
    println!("remove_command is not yet implemented");
    Ok(())
}