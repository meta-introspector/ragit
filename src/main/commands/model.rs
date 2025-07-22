use crate::prelude::*;

use crate::commands::model_command::model_command;

pub async fn model_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    model_command(args, pre_args).await
}