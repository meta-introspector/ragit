use crate::prelude::*;
use ragit_utils::index::commands::summary::summary_command;

pub async fn summary_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    summary_command(args, pre_args).await
}