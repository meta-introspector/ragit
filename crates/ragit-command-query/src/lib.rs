use ragit_utils::prelude::*;
//use ragit_cli::prelude::*;
use ragit_index_io::index_struct::load_index_from_path;
//use ragit_index_core::Index;
use ragit_index_io::index_struct::Index;
use ragit_utils::project_root::find_root;
use ragit_utils::doc_utils::get_doc_content;
use ragit_types::query::QueryTurn;
use serde_json::Value;

pub async fn query_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--no-pdl"])
        .optional_flag(&["--multi-turn"])
        .optional_flag(&["--json"])
        .short_flag(&["--json"])
        .args(ArgType::Query, ArgCount::Geq(1))
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/query.txt"));
        return Ok(());
    }

    let mut index = load_index_from_path(&find_root()?)?;

    let no_pdl = parsed_args.get_flag(0).is_some();
    let multi_turn = parsed_args.get_flag(1).is_some();
    let json_mode = parsed_args.get_flag(2).is_some();
    let query = parsed_args.get_args().join(" ");

    if multi_turn {
        let mut turns = vec![];

        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input == ".exit" {
                break;
            }

            let response = index.query(&input, turns.clone(), None).await?;
            println!("{}", response.get_message());
            turns.push(QueryTurn {
                query: input.to_string(),
                response: response.get_message().to_string(),
            });
        }
    } else {
        let response = index.query(&query, vec![], None).await?;

        if json_mode {
            println!("{}", serde_json::to_string_pretty(&response.to_json())?);
        } else {
            println!("{}", response.get_message());
        }
    }

    Ok(())
}
