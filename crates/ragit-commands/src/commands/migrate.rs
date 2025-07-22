use crate::prelude::*;

pub async fn migrate_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/migrate.txt"));
    Err(Error::CliError(CliError::new_message("migrate command is not implemented yet".to_string())))
}