use crate::prelude::*;
use ragit_utils::index::commands::merge::merge_command;

pub async fn merge_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    merge_command(args, pre_args).await
}