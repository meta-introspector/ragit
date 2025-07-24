use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoveResult {
    pub removed_files: usize,
    pub removed_chunks: usize,
}

pub async fn remove_command(
    _args: Vec<String>,
    _pre_args: crate::cli_types::ParsedArgs,
) -> Result<(), Error> {
    println!("remove_command is not yet implemented");
    Ok(())
}
