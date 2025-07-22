use crate::prelude::*;

use crate::prelude::*;

use crate::commands::config_command::config_command;

pub async fn config_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    config_command(args, pre_args).await
}