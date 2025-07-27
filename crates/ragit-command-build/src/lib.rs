use ragit_utils::prelude::*;
use ragit_index_effects::build as build_index;
use ragit_utils::project_root::find_root;
use ragit_utils::doc_utils::get_doc_content;
use ragit_index_types::index_struct::Index;
use ragit_index_types::load_mode::LoadMode;

pub async fn build_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    let parsed_args = ArgParser::new()
        .arg_flag_with_default(
            "--jobs",
            "8",
            ArgType::IntegerBetween {
                min: Some(0),
                max: None,
            },
        )
        .optional_flag(&["--quiet"])
        .short_flag(&["--quiet"])
        .parse(&args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/build.txt"));
        return Ok(());
    }

    let jobs = parsed_args
        .arg_flags
        .get("--jobs")
        .as_ref()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let quiet = parsed_args.get_flag(0).is_some();
    let mut index = Index::load(find_root()?, LoadMode::OnlyJson)?;
    build_index(&mut index, jobs, quiet).await?;
    Ok(())
}
