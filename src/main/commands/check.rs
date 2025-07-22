use crate::prelude::*;

use crate::main::commands::check_command::check_command;

pub async fn check_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    check_command(args, pre_args).await
}