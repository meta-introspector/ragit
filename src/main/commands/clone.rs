use crate::prelude::*;
use ragit_utils::index::commands::clone::clone_command;

pub async fn clone_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    clone_command(args, pre_args).await
}