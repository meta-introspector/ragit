use crate::prelude::*;
use ragit_utils::index::commands::pull::pull_command;

pub async fn pull_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    pull_command(args, pre_args).await
}