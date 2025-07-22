use crate::prelude::*;
use crate::BuildOptions;

pub async fn build_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    crate::index::commands::build::build_command(args, pre_args).await
}