use crate::prelude::*;
use crate::commands::version_command::version_command;

pub async fn version_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    version_command(args, pre_args).await
}