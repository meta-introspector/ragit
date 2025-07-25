use ragit_utils::prelude::*;
use ragit_api::prelude::*;
use ragit_types::prelude::*;
use ragit_index_io::load_index_from_path;
use ragit_index_core::Index;
use ragit_utils::project_root::find_root;
use ragit_utils::doc_utils::get_doc_content;
use serde_json::Value;

pub async fn extract_keywords_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--full-schema"])
        .optional_flag(&["--json"])
        .short_flag(&["--json"])
        .args(ArgType::Query, ArgCount::Exact(1))
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/extract-keywords.txt"));
        return Ok(());
    }

    let index = load_index_from_path(&find_root()?)?;
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
            println!("{:?}", keywords);
        } else {
            println!("{}", keywords.join("\n"));
        }
    }

    Ok(())
}