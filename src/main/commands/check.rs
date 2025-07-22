use crate::prelude::*;

pub async fn check_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    crate::index::commands::check::check_command(args, pre_args).await
}