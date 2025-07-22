use crate::prelude::*;

use crate::main::commands::cat_file_command::cat_file_command;

pub async fn cat_file_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    cat_file_command(args, pre_args).await
}