use crate::prelude::*;

use ragit_utils::index::commands::muse_summarize::muse_summarize_command;

pub async fn muse_summarize_command_main(args: Vec<String>, pre_args: ragit_args::ParsedArgs) -> Result<(), Error> {
    muse_summarize_command(args, pre_args).await
}