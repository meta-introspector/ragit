use crate::prelude::*;
use ragit_utils::index::commands::pdl::pdl_command;

pub async fn pdl_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    pdl_command(args, pre_args).await
}