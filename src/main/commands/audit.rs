use crate::prelude::*;
use ragit_utils::index::commands::audit::audit_command;

pub async fn audit_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    audit_command(args, pre_args).await
}