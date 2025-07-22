use crate::prelude::*;

use crate::main::commands::clone_command::clone_command;

pub async fn clone_command_main(args: Vec<String>, _pre_args: ParsedArgs) -> Result<(), Error> {
    clone_command(&args).await
}