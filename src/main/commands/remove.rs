use crate::prelude::*;
use crate::commands::remove_command::remove_command;

pub async fn remove_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    remove_command(args, pre_args).await
}