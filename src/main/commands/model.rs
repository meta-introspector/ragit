use crate::prelude::*;

pub async fn model_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    crate::index::commands::model::model_command(args, pre_args).await
}