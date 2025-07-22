use crate::prelude::*;

use crate::commands::muse_summarize_command::muse_summarize_command;

pub async fn muse_summarize_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    muse_summarize_command(args, pre_args).await
}