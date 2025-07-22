use crate::prelude::*;
use crate::commands::qa_tune_command::qa_tune_command;

pub async fn qa_tune_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    qa_tune_command(args, pre_args).await
}