use ragit_utils::prelude::*;
use ragit_index_query::query;
use ragit_index_types::index_struct::Index;
use ragit_index_types::load_mode::LoadMode;

use ragit_utils::project_root::find_root;
use ragit_utils::doc_utils::get_doc_content;
use ragit_types::query_turn::QueryTurn;
use std::path::PathBuf;
use ragit_memory_monitor::MemoryMonitor;

pub async fn query_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    let mut memory_monitor = MemoryMonitor::new(false, None, None);
    let parsed_args = ArgParser::new()
        .optional_flag(&["--no-pdl"])
        .optional_flag(&["--multi-turn"])
        .optional_flag(&["--json"])
        .short_flag(&["--json"])
        .args(ArgType::Query, ArgCount::Geq(1))
        .optional_arg_flag("--include", ArgType::String)
        .optional_arg_flag("--path", ArgType::Path)
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/query.txt"));
        return Ok(());
    }

    let _no_pdl = parsed_args.get_flag(0).is_some();
    let multi_turn = parsed_args.get_flag(1).is_some();
    let json_mode = parsed_args.get_flag(2).is_some();
    let query_str = parsed_args.get_args().join(" ");
    let _include_pattern = parsed_args.arg_flags.get("--include").map(|s| s.to_string());
    let kb_path = parsed_args.arg_flags.get("--path").map(|s| s.to_string());

    let index = if let Some(path_str) = kb_path {
        Index::load(PathBuf::from(path_str), LoadMode::OnlyJson)?
    } else {
        Index::load(find_root()?, LoadMode::OnlyJson)?
    };

    if multi_turn {
        let mut turns = vec![];

        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input == ".exit" {
                break;
            }

            let response = query(&index, &input, turns.clone(), None, &mut memory_monitor).await?;
            println!("{}", response.get_message());
            turns.push(QueryTurn {
                query: input.to_string(),
                response: ragit_types::query_turn::QueryResponse { response: response.get_message().to_string() },
            });
        }
    } else {
        let response = query(&index, &query_str, vec![], None, &mut memory_monitor).await?;

        if json_mode {
            let mut search_results = Vec::new();
            for chunk in response.retrieved_chunks {
                search_results.push(serde_json::json!({
                    "file_path": chunk.file,
                    "line_number": chunk.index + 1, // Assuming index is 0-based, line numbers are 1-based
                    "line_content": chunk.data
                }));
            }
            println!("{}", serde_json::to_string_pretty(&search_results)?);
        } else {
            println!("{}", response.get_message());
        }
    }

    Ok(())
}