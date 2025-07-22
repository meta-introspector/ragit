use crate::prelude::*;
use crate::commands::pdl_command::pdl_command;

pub async fn pdl_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    pdl_command(args, pre_args).await
}