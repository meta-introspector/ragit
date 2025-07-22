use crate::prelude::*;
use ragit_utils::index::commands::archive::{archive_create_command, archive_extract_command};

pub async fn archive_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    let command = args.get(2).map(|arg| arg.as_str());

    match command {
        Some("create") => archive_create_command(args, pre_args).await?,
        Some("extract") => archive_extract_command(args, pre_args).await?,
        _ => {
            return Err(Error::CliError {
                message: String::from("Unknown archive command."),
                span: (String::new(), 0, 0),
            });
        }
    }
    Ok(())
}