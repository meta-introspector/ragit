use crate::prelude::*;

use ragit_utils::index::commands::init::init_command;

pub async fn init_command_main(args: Vec<String>, pre_args: ragit_args::ParsedArgs) -> Result<(), Error> {
    init_command(args, pre_args).await
}