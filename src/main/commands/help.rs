use crate::prelude::*;

pub async fn help_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    crate::index::commands::help::help_command(args, pre_args).await
}