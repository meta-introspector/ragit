use crate::prelude::*;
use crate::commands::query_command::query_command;

pub async fn query_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    query_command(args, pre_args).await
}