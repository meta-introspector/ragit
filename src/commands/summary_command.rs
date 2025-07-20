use crate::{Error, Index, LoadMode, SummaryMode, UidQueryConfig};
use ragit_cli::{ArgCount, ArgParser, ArgType};

pub async fn summary_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--rerank"])
        .optional_flag(&["--quiet"])
        .short_flag(&["--quiet"])
        .args(ArgType::UidOrPath, ArgCount::Any)
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../docs/commands/summary.txt"));
        return Ok(());
    }

    let mut index = Index::load(crate::find_root()?, LoadMode::QuickCheck)?;
    let summary_mode = if parsed_args.get_flag(0).is_some() {
        SummaryMode::Rerank
    } else {
        SummaryMode::Simple
    };
    let quiet = parsed_args.get_flag(1).is_some();
    let query = parsed_args.get_args();

    let summary = index.summary(&query, summary_mode, quiet).await?;

    println!("{}", summary.unwrap_or_default());

    Ok(())
}
