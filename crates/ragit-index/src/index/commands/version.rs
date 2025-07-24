use crate::prelude::*;

pub async fn version_command(
    _args: Vec<String>,
    _pre_args: ParsedArgs,
) -> Result<(), ApiError> {
    println!("version_command is not yet implemented");
    Ok(())
}