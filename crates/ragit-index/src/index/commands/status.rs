use crate::prelude::*;

pub async fn status_command(
    _args: Vec<String>,
    _pre_args: ParsedArgs,
) -> Result<(), ApiError> {
    println!("status_command is not yet implemented");
    Ok(())
}