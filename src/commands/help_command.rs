use crate::Error;
use ragit_cli::{ArgCount, ArgParser, ArgType};

pub async fn help_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .args(ArgType::Command, ArgCount::Leq(1))
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../docs/commands/help.txt"));
        return Ok(());
    }

    match parsed_args.get_args().get(0).map(|arg| arg.as_str()) {
        Some("chunks") => {
            println!("{}", include_str!("../../docs/chunks.md"));
        }
        Some("config-reference") => {
            println!("{}", include_str!("../../docs/config.md"));
        }
        Some("pdl-format") => {
            println!("{}", include_str!("../../docs/pdl_format.md"));
        }
        Some("pipeline") => {
            println!("{}", include_str!("../../docs/pipeline.md"));
        }
        Some("quick-guide") => {
            println!("{}", include_str!("../../docs/quick_guide.md"));
        }
        Some("uid-query") => {
            println!("{}", include_str!("../../docs/uid_query.md"));
        }
        Some(command) => {
            let mut new_args = args.to_vec();
            new_args[1] = command.to_string();
            new_args[2] = String::from("--help");
            return crate::run(new_args).await;
        }
        None => {
            println!("{}", include_str!("../../docs/intro.txt"));
        }
    }

    Ok(())
}
