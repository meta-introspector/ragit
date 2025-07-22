use crate::prelude::*;

pub async fn config_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    crate::index::commands::config::config_command(args, pre_args).await
}