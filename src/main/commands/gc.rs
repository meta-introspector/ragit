use crate::prelude::*;

use ragit_utils::index::commands::gc::gc_command;

pub async fn gc_command_main(args: Vec<String>, pre_args: ragit_args::ParsedArgs) -> Result<(), Error> {
    gc_command(args, pre_args).await
}