use crate::prelude::*;
use crate::commands::status_command::status_command;

pub async fn status_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    status_command(args, pre_args).await
}