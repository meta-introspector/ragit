use crate::prelude::*;

use ragit_utils::index::commands::model::model_command;

pub async fn model_command_main(args: Vec<String>, pre_args: ragit_args::ParsedArgs) -> Result<(), Error> {
    model_command(args, pre_args).await
}