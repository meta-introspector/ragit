use crate::prelude::*;

pub async fn gc_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    crate::index::commands::gc::gc_command(args, pre_args).await
}