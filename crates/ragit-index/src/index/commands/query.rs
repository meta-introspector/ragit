use crate::prelude::*;
use ragit_utils::cli_types::ParsedArgs;

pub async fn query_command(
    _args: Vec<String>,
    _pre_args: crate::cli_types::ParsedArgs,
) -> Result<(), Error> {
    println!("query_command is not yet implemented");
    Ok(())
}
