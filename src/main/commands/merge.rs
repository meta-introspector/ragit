use crate::prelude::*;

pub async fn merge_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    crate::index::commands::merge::merge_command(args, pre_args).await
}