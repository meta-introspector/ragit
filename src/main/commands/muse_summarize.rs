use crate::prelude::*;

pub async fn muse_summarize_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    crate::index::commands::muse_summarize::muse_summarize_command(args, pre_args).await
}