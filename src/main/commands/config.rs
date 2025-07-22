use crate::prelude::*;

use ragit_utils::index::commands::config::config_command;

pub async fn config_command_main(args: Vec<String>, pre_args: ragit_args::ParsedArgs) -> Result<(), Error> {
    config_command(args, pre_args).await
}