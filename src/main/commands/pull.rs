use crate::prelude::*;
use crate::main::find_root;
use crate::main::commands::pull_command;

pub async fn pull_command_main(args: Vec<String>, _pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    let root_dir = find_root()?;
    pull_command(root_dir, &args).await
}