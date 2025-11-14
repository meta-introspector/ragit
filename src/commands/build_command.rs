use ragit_commands::build_command_main;
use ragit_commands::ParsedArgs;
use ragit_utils::error::Error;

pub async fn build_command(args: &[String]) -> Result<(), Error> {
    let pre_args = ParsedArgs::new_from_args(args.to_vec());
    build_command_main(args.to_vec(), pre_args).await
}