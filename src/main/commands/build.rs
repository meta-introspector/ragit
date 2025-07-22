use crate::prelude::*;
use crate::BuildOptions;

use ragit_utils::index::commands::build::build_command;

pub async fn build_command_main(args: Vec<String>, pre_args: ragit_args::ParsedArgs) -> Result<(), Error> {
    build_command(args, pre_args).await
}