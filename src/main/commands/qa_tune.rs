use crate::prelude::*;
use ragit_utils::index::commands::qa_tune_command;

pub async fn qa_tune_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    qa_tune_command(args, pre_args).await
}