use crate::prelude::*;

pub async fn status_command(
    _args: Vec<String>,
    _pre_args: crate::cli_types::ParsedArgs,
) -> Result<(), Error> {
    println!("status_command is not yet implemented");
    Ok(())
}
