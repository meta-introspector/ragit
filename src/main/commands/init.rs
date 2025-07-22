use crate::prelude::*;

pub async fn init_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    crate::index::commands::init::init_command(args, pre_args).await
}