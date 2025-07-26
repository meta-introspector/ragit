use ragit_utils::prelude::*;
use ragit_api::prelude::*;
use ragit_types::prelude::*;
use ragit_utils::doc_utils::get_doc_content;
use std::path::PathBuf;
//use ragit_index_core::Index;
//use ragit_index_io::index_struct::Index;
use ragit_index_types::index_struct::Index;
use ragit_index_core::load_index_from_path;

pub async fn clone_command_main(args: &[String]) -> Result<(), anyhow::Error> {
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

    let mut index = Index::dummy();
    index.clone(url, depth).await?;

    Ok(())
}
