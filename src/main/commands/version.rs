use crate::prelude::*;
use ragit_utils::index::commands::version::version_command;

pub async fn version_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    version_command(args, pre_args).await
}