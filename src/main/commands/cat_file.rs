use crate::prelude::*;

use ragit_utils::index::commands::cat_file::cat_file_command;

pub async fn cat_file_command_main(args: Vec<String>, pre_args: ragit_args::ParsedArgs) -> Result<(), Error> {
    cat_file_command(args, pre_args).await
}