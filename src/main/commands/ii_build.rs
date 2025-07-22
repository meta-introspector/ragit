use crate::prelude::*;
use ragit_utils::index::commands::ii_build::ii_command;

pub async fn ii_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    ii_command(args, pre_args).await
}