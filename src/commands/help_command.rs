use ragit_commands::help_command_main;
use ragit_utils::error::Error;

pub async fn help_command(args: &[String]) -> Result<(), Error> {
    help_command_main(args).await
}