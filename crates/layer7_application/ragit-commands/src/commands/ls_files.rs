use crate::Error;
// use crate::prelude::*;
// use ragit_utils::cli_types::{ArgParser, ArgType, ArgCount};
// use ragit_utils::doc_utils::get_doc_content;
// use ragit_query::UidQueryConfig;

pub async fn ls_files_command_main(_args: &[String]) -> Result<(), Error> {
    // let parsed_args = ArgParser::new()
    //     .optional_flag(&["--processed", "--staged"])
    //     .optional_flag(&["--json"])
    //     .short_flag(&["--json"])
    //     .args(ArgType::UidOrPath, ArgCount::Any)
    //     .parse(args, 2)?;

    // if parsed_args.show_help() {
    //     println!("{}", get_doc_content("commands/ls-files.txt"));
    //     return Ok(());
    // }

    // let processed = parsed_args.get_flag(0).unwrap_or_default() == "--processed";
    // let staged = parsed_args.get_flag(0).unwrap_or_default() == "--staged";
    // let json_mode = parsed_args.get_flag(1).is_some();
    // let index = Index::load(find_root()?.into(), LoadMode::OnlyJson)?;
    // let args = parsed_args.get_args();
    // let mut files = if args.is_empty() {
    //     if !json_mode {
    //         println!("{} files", index.file_count);
    //     }
    //     index.list_files(
    //         &|_| true,  // no filter
    //         &|f| f.clone(),  // no map
    //         &|file| file.path.to_string_lossy().to_string(),  // sort by path
    //     )?
    // } else {
    //     let query = uid_query(&index, &args, QueryConfig::new().file_only())?;
    //     let mut processed_files_len = 0;
    //     let mut staged_files_len = 0;
    //     let mut files = vec![];
    //     if processed {
    //         for (path, uid) in query.get_processed_files() {
    //             processed_files_len += 1;
    //             files.push(index.get_file_schema(Some(path.into()), Some(uid))?);
    //         }
    //     }

    //     if staged {
    //         for path in query.get_staged_files() {
    //             staged_files_len += 1;
    //             files.push(index.get_file_schema(Some(path.into()), None)?);
    //         }
    //     }

    //     if files.is_empty() {
    //         return Err(Error::CliError(CliError::new_message(format!("There's no file that matches `{}`.", args.join(" ")))));
    //     }
    //     files
    // };

    // if json_mode {
    //     println!("{}", serde_json::to_string_pretty(&files)?);
    // } else {
    //     for file in files {
    //         println!("{}", file.path.to_string());
    //     }
    // }

    Ok(())
}
