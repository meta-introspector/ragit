use crate::prelude::*;

pub async fn help_command_main(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .args(ArgType::Command, ArgCount::Leq(1))
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/help.txt"));
        return Ok(());
    }

    match parsed_args.get_args().get(0).map(|arg| arg.as_str()) {
        Some("chunks") => {
            println!("{}", get_doc_content("chunks.md"));
        }
        Some("config-reference") => {
            println!("{}", get_doc_content("config.md"));
        }
        Some("pdl-format") => {
            println!("{}", get_doc_content("pdl_format.md"));
        }
        Some("pipeline") => {
            println!("{}", get_doc_content("pipeline.md"));
        }
        Some("quick-guide") => {
            println!("{}", get_doc_content("quick_guide.md"));
        }
        Some("uid-query") => {
            println!("{}", get_doc_content("uid_query.md"));
        }
        Some(command) => {
            let mut new_args = args.to_vec();
            new_args[1] = command.to_string();
            new_args[2] = String::from("--help");
            return run_command_main(&new_args).await;
        }
        None => {
            println!("{}", get_doc_content("intro.txt"));
        }
    }

    Ok(())
}