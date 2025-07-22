use crate::prelude::*;
use ragit_utils::index::commands::push::push_command;

pub async fn push_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    push_command(args, pre_args).await
}