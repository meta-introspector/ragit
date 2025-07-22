use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct VersionInfo {
    pub version: String,
    pub compatible: bool,
}

pub fn get_compatibility_warning(version_info: &VersionInfo) -> Option<String> {
    None
}

pub async fn version_command(args: Vec<String>, pre_args: ragit_args::ParsedArgs) -> Result<(), Error> {
    println!("version_command is not yet implemented");
    Ok(())
}