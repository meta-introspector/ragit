use crate::prelude::*;

use crate::prelude::*;

use crate::commands::ls_chunks_command::ls_chunks_command;
use crate::commands::ls_files_command::ls_files_command;
use crate::commands::ls_images_command::ls_images_command;
use crate::commands::ls_models_command::ls_models_command;
use crate::commands::ls_terms_command::ls_terms_command;

pub async fn ls_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    let command = args.get(2).map(|arg| arg.as_str());

    match command {
        Some("chunks") => ls_chunks_command(args, pre_args).await?,
        Some("files") => ls_files_command(args, pre_args).await?,
        Some("images") => ls_images_command(args, pre_args).await?,
        Some("models") => ls_models_command(args, pre_args).await?,
        Some("terms") => ls_terms_command(args, pre_args).await?,
        _ => {
            return Err(Error::CliError(ragit_utils::error::CliError::new_message("Unknown ls command.".to_string())));
        }
    }
    Ok(())
}