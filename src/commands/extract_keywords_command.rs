use crate::{Error, Index, LoadMode};
use ragit_cli::{ArgCount, ArgParser, ArgType};

pub async fn extract_keywords_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--full-schema"])
        .optional_flag(&["--json"])
        .short_flag(&["--json"])
        .args(ArgType::Query, ArgCount::Exact(1))
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../docs/commands/extract-keywords.txt"));
        return Ok(());
    }

    let index = Index::load(crate::find_root()?.to_string_lossy().into_owned(), LoadMode::OnlyJson)?;
    let full_schema = parsed_args.get_flag(0).is_some();
    let json_mode = parsed_args.get_flag(1).is_some();
    let query = &parsed_args.get_args_exact(1)?[0];
    let result = index.extract_keywords(query).await?;

    if full_schema {
        if json_mode {
            println!("{}", serde_json::to_string_pretty(&result)?);
        } else {
            println!("keywords:");
            println!(
                "{}",
                result
                    .keywords
                    .iter()
                    .map(|keyword| format!("    {keyword}"))
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
            println!("extra:");
            println!(
                "{}",
                result
                    .extra
                    .iter()
                    .map(|extra| format!("    {extra}"))
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
        }
    } else {
        let mut keywords = result.keywords.clone();

        for e in result.extra.into_iter() {
            if !keywords.contains(&e) {
                keywords.push(e);
            }
        }

        if json_mode {
            println!("{keywords:?}");
        } else {
            println!("{}", keywords.join("\n"));
        }
    }

    Ok(())
}
