use ragit_utils::prelude::*;
use ragit_api::prelude::*;
use ragit_types::prelude::*;

pub async fn clone_command_main(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .optional_arg_flag(
            "--depth",
            ArgType::IntegerBetween {
                min: Some(0),
                max: None,
            },
        )
        .args(ArgType::Path, ArgCount::Exact(1))
        .parse(args, 1)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/clone.txt"));
        return Ok(());
    }

    let url = parsed_args.get_args_exact(1)?.get(0).unwrap();
    let depth = parsed_args
        .arg_flags
        .get("--depth")
        .map(|s| s.parse::<usize>().unwrap());

    let mut index = Index::new(PathBuf::from("."))?;
    index.clone(url, depth).await?;

    Ok(())
}
