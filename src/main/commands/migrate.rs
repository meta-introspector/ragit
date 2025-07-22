use crate::prelude::*;
use ragit_utils::index::commands::migrate::migrate_command;

pub async fn migrate_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    migrate_command(args, pre_args).await
}