use crate::prelude::*;

use crate::prelude::*;

use crate::commands::meta_command::meta_command;

pub async fn meta_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    meta_command(args, pre_args).await
}