use crate::prelude::*;

use crate::prelude::*;

use crate::commands::gc_command::gc_command;

pub async fn gc_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    gc_command(args, pre_args).await
}