use crate::prelude::*;

use ragit_utils::index::commands::help::help_command;

pub async fn help_command_main(args: Vec<String>, pre_args: ragit_args::ParsedArgs) -> Result<(), Error> {
    help_command(args, pre_args).await
}