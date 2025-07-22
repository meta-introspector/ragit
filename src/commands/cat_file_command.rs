use ragit_utils::error::Error;
use ragit_utils::index::index_struct::Index;
use ragit_utils::index::load_mode::LoadMode;
use std::path::PathBuf;
use ragit_utils::uid::query_helpers::UidQueryConfig;
use ragit_utils::chunk::utils::into_multi_modal_contents;
use ragit_utils::project_root::find_root;
use ragit_cli::{ArgCount, ArgParser, ArgType};
use ragit_pdl::encode_base64;
use std::io::Write;

pub async fn cat_file_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--json"])
        .short_flag(&["--json"])
        .args(ArgType::UidOrPath, ArgCount::Exact(1))
        .parse(&args, 2)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../docs/commands/cat-file.txt"));
        return Ok(());
    }

    let index = Index::load(find_root()?.to_string_lossy().into_owned(), LoadMode::OnlyJson)?;
    let query = parsed_args.get_args_exact(1)?.clone();
    let query_result = index.uid_query(&args, UidQueryConfig::new())?;
    let json_mode = parsed_args.get_flag(0).is_some();

    if query_result.has_multiple_matches() {
        return Err(Error::UidQueryError(format!("There're multiple file/chunk that match `{}`. Please give more specific query.", query[0])));
    }

    else if let Some(uid) = query_result.get_chunk_uid() {
        let chunk = index.get_chunk_by_uid(uid)?;

        if json_mode {
            println!("{}", serde_json::to_string_pretty(&into_multi_modal_contents(&chunk.data, &chunk.images))?);
        }

        else {
            println!("{}", chunk.data);
        }
    }

    else if let Some((_, uid)) = query_result.get_processed_file() {
        let chunk = index.get_merged_chunk_of_file(uid)?;

        if json_mode {
            println!("{}", serde_json::to_string_pretty(&chunk.raw_data)?);
        }

        else {
            println!("{}", chunk.human_data);
        }
    }

    else if let Some(f) = query_result.get_staged_file() {
        return Err(Error::UidQueryError(format!("`{f}` has no chunks yet. Please run `rag build`.")));
    }

    else if let Some(image_uid) = query_result.get_image_uid() {
        let image = index.get_image_schema(image_uid, true)?;

        if json_mode {
            println!("{:?}", encode_base64(&image.bytes));
        }

        else {
            std::io::stdout().write_all(&image.bytes)?;
        }
    }

    else {
        return Err(Error::UidQueryError(format!("There's no chunk/file/image that matches `{}`.", query[0])));
    }
    Ok(())
}
