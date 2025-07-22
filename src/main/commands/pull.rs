use crate::prelude::*;
use crate::main::find_root;
use crate::commands::pull_command::pull_command;

pub async fn pull_command_main(args: Vec<String>, _pre_args: ParsedArgs) -> Result<(), Error> {
    let root_dir = find_root()?;
    pull_command(root_dir, &args).await
}