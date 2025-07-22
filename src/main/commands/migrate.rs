use crate::prelude::*;

use crate::prelude::*;

use crate::commands::migrate_command::migrate_command;

pub async fn migrate_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    migrate_command(args, pre_args).await
}