use crate::prelude::*;

pub async fn cat_file_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    crate::index::commands::cat_file::cat_file_command(args, pre_args).await
}