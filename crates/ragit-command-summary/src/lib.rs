use ragit_utils::prelude::*;
//use ragit_cli::prelude::*;
//use ragit_index_io::load_index_from_path;
//use ragit_index_core::Index;
use ragit_utils::project_root::find_root;
use ragit_utils::doc_utils::get_doc_content;
//use ragit_types::summary::SummaryMode;

pub async fn summary_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--rerank"])
        .optional_flag(&["--quiet"])
        .short_flag(&["--quiet"])
        .args(ArgType::UidOrPath, ArgCount::Any)
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/summary.txt"));
        return Ok(());
    }

    let mut index = load_index_from_path(&find_root()?)?;

    let summary_mode = if parsed_args.get_flag(0).is_some() {
        SummaryMode::Rerank
    } else {
        SummaryMode::Simple
    };
    let quiet = parsed_args.get_flag(1).is_some();
    let query = parsed_args.get_args();

    let summary = index.summary(Some(summary_mode)).await?;

    println!("{}", summary.unwrap_or_default());

    Ok(())
}
