use crate::prelude::*;
use ragit_utils::index::commands::build::build_command;

pub async fn build_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    build_command(args, pre_args).await
}