use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct VersionInfo {
    pub version: String,
    pub compatible: bool,
}

pub fn get_compatibility_warning(_version_info: &VersionInfo) -> Option<String> {
    None
}

pub async fn version_command(
    _args: Vec<String>,
    _pre_args: crate::cli_types::ParsedArgs,
) -> Result<(), Error> {
    println!("version_command is not yet implemented");
    Ok(())
}
