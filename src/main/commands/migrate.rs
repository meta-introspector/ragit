use crate::prelude::*;

pub async fn migrate_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    crate::index::commands::migrate::migrate_command(args, pre_args).await
}