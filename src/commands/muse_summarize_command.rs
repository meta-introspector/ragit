use crate::Error;
use crate::Index;
use crate::LoadMode;
use crate::SummaryMode;
use ragit_cli::{ArgCount, ArgParser, ArgType};

pub async fn muse_summarize_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--force", "--cached"])
        .parse(args, 2)?;

    if parsed_args.show_help() {
        // TODO: create a help file for muse-summarize
        println!("Usage: rag muse-summarize [--force | --cached]");
        return Ok(());
    }

    let root_dir = crate::find_root()?;
    let mut index = Index::load(crate::find_root()?.to_string_lossy().into_owned(), LoadMode::QuickCheck)?;
    index.api_config.enable_muse_mode = true;
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
