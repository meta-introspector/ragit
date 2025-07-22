use crate::prelude::*;

use crate::prelude::*;

use crate::commands::help_command::help_command;

pub async fn help_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    help_command(args, pre_args).await
}