use crate::prelude::*;

use ragit_utils::index::commands::meta::meta_command;

pub async fn meta_command_main(args: Vec<String>, pre_args: ragit_args::ParsedArgs) -> Result<(), Error> {
    meta_command(args, pre_args).await
}