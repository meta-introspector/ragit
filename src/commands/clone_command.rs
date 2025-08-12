use ragit_commands::clone_command_main;
use ragit_utils::error::Error;

pub async fn clone_command(args: &[String]) -> Result<(), Error> {
    clone_command_main(args).await
}