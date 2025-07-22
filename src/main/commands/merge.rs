use crate::prelude::*;

use crate::prelude::*;

use crate::commands::merge_command::merge_command;

pub async fn merge_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    merge_command(args, pre_args).await
}