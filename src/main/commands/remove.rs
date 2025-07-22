use crate::prelude::*;
use ragit_utils::index::commands::remove_command;

pub async fn remove_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    remove_command(args, pre_args).await
}