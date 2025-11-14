use ragit_commands::extract_keywords_command_main;
use ragit_utils::error::Error;

pub async fn extract_keywords_command(args: &[String]) -> Result<(), Error> {
    extract_keywords_command_main(args).await
}