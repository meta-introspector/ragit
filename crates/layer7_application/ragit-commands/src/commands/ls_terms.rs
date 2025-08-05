use crate::prelude::*;
use crate::prelude::*;
use ragit_utils::cli_types::{ArgParser, ArgType, ArgCount};
use ragit_utils::doc_utils::get_doc_content;
use ragit_query::UidQueryConfig;
use ragit_types::processed_doc::ProcessedDoc;

pub async fn ls_terms_command_main(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--term-only", "--stat-only"])
        .optional_flag(&["--json"])
        .short_flag(&["--json"])
        .args(ArgType::UidOrPath, ArgCount::Any)
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/ls-terms.txt"));
        return Ok(());
    }
    let index = Index::load(find_root()?.into(), LoadMode::OnlyJson)?;
    let term_only = parsed_args.get_flag(0).unwrap_or_default() == "--term-only";
    let stat_only = parsed_args.get_flag(0).unwrap_or_default() == "--stat-only";
    let json_mode = parsed_args.get_flag(1).is_some();
    let args = parsed_args.get_args();

    let processed_doc = if args.is_empty() {
        let mut result = ProcessedDoc::empty();

        for chunk_uid in index.get_all_chunk_uids()? {
            result.extend(&index.get_tfidf_by_chunk_uid(chunk_uid)?);
        }

        result
    } else {
        let query = uid_query(&index, &args, ragit_query::UidQueryConfig::new().file_or_chunk_only().no_staged_file())?;

        if query.has_multiple_matches() {
            return Err(Error::CliError(CliError::new_message(format!(
                "There're {} chunks/files that match `{}`. Please give more specific query.",
                query.len(),
                args.join(" ")
            ))));
        } else if query.is_empty() {
            return Err(Error::CliError(CliError::new_message(format!(
                "There's no chunk or file that matches `{}`.",
                args.join(" ")
            ))));
        } else if let Some((_, uid)) = query.get_processed_file() {
            index.get_tfidf_by_file_uid(uid)?
        } else if let Some(uid) = query.get_chunk_uid() {
            index.get_tfidf_by_chunk_uid(uid)?
        } else {
            unreachable!()
        }
    };

    println!("{}", processed_doc.render(term_only, stat_only, json_mode));
    Ok(())
}