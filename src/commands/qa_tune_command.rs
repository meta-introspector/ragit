use ragit_commands::{Error, qa_tune_command_main};

pub async fn qa_tune_command(args: &[String]) -> Result<(), Error> {
    qa_tune_command_main(args).await
}