use ragit_commands::{Error, muse_summarize_command_main};

pub async fn muse_summarize_command(args: &[String]) -> Result<(), Error> {
    muse_summarize_command_main(args).await
}