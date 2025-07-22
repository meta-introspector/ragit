use crate::prelude::*;

use crate::main::commands::build_command::build_command;

pub async fn build_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    build_command(args, pre_args).await
}