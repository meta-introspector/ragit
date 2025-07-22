use crate::prelude::*;
use ragit_utils::index::commands::status_command;

pub async fn status_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    status_command(args, pre_args).await
}