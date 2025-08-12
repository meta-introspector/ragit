use ragit_commands::{Error, migrate_command_main};

pub fn migrate_command(args: &[String]) -> Result<(), Error> {
    migrate_command_main(args).map_err(|e| e.into())
}