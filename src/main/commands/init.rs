use crate::prelude::*;

use crate::prelude::*;

use crate::commands::init_command::init_command;

pub async fn init_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    init_command(args, pre_args).await
}