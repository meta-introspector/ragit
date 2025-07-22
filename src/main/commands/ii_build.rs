use crate::prelude::*;

use crate::prelude::*;

use crate::commands::ii_command::ii_command;

pub async fn ii_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    ii_command(args, pre_args).await
}