use crate::prelude::*;
use ragit_utils::index::commands::query::query_command;

pub async fn query_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    query_command(args, pre_args).await
}