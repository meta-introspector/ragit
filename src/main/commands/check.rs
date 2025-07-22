use crate::prelude::*;
use ragit_utils::index::commands::check::check_command;

pub async fn check_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    check_command(args, pre_args).await
}