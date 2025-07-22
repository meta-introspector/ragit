use ragit_commands::{Error, summary_command_main};

pub async fn summary_command(args: &[String]) -> Result<(), Error> {
    summary_command_main(args).await
}