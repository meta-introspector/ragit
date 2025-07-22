use crate::prelude::*;

pub async fn ii_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    crate::index::commands::ii_build::ii_command(args, pre_args).await
}