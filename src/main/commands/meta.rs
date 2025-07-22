use crate::prelude::*;

pub async fn meta_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    crate::index::commands::meta::meta_command(args, pre_args).await
}