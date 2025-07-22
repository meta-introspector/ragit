use crate::prelude::*;
use crate::commands::summary_command::summary_command;

pub async fn summary_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    summary_command(args, pre_args).await
}