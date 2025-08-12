use crate::prelude::*;

pub async fn extract_keywords_command_main(args: &[String]) -> Result<(), Error> {
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

    let index = Index::load(find_root()?.into(), LoadMode::OnlyJson)?;
    let full_schema = parsed_args.get_flag(0).is_some();
    let json_mode = parsed_args.get_flag(1).is_some();
    let query = &parsed_args.get_args_exact(1)?[0];
    let result = index.extract_keywords(query.to_string()).await?;

    if full_schema {
        if json_mode {
            println!("{}", serde_json::to_string_pretty(&result)?);
        } else {
            println!("keywords:");
            println!(
                "{}",
                result.0
                    .iter()
                    .map(|keyword| format!("    {keyword}"))
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
        }
    } else {
        let mut keywords = result.0.clone();

        for e in result.0.into_iter() {
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
