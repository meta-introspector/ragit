use crate::prelude::*;

pub async fn clone_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    crate::index::commands::clone::clone_command(args, pre_args).await
}