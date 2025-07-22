use crate::prelude::*;

pub async fn ls_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    let command = args.get(2).map(|arg| arg.as_str());

    match command {
        Some("chunks") => crate::index::commands::ls::ls_chunks_command(args, pre_args).await?,
        Some("files") => crate::index::commands::ls::ls_files_command(args, pre_args).await?,
        Some("images") => crate::index::commands::ls::ls_images_command(args, pre_args).await?,
        Some("models") => crate::index::commands::ls::ls_models_command(args, pre_args).await?,
        Some("terms") => crate::index::commands::ls::ls_terms_command(args, pre_args).await?,
        _ => {
            return Err(Error::CliError {
                message: String::from("Unknown ls command."),
                span: (String::new(), 0, 0),
            });
        }
    }
    Ok(())
}