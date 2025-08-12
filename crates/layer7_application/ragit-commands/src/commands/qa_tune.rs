use crate::prelude::*;

pub async fn qa_tune_command_main(_args: &[String]) -> Result<(), Error> {
    // let parsed_args = ArgParser::new()
    //     .args(ArgType::Query, ArgCount::Any)
    //     .optional_flag(&["--json"])
    //     .parse(args, 2)?;

    // if parsed_args.show_help() {
    //     println!("{}", get_doc_content("commands/qa-tune.txt"));
    //     return Ok(());
    // }

    // let result_file = find_root()?.join(".ragit").join("qa_results.json");
    // let mut results: Vec<QueryResponse> = serde_json::from_str(&fs_read_string(&result_file)?)?;

    // for result in results.iter_mut() {
    //     let updated_log = serde_json::to_string_pretty(&results)?;
    //     fs_write_string(&result_file.to_string_lossy(), &updated_log, FsWriteMode::CreateOrTruncate)?;
    // }

    Ok(())
}
