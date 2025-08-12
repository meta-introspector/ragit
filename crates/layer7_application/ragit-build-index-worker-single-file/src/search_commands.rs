use anyhow::Result;
use crate::args::search_args::SearchArgs;
use ragit_command_query::query_command_main;

pub async fn search_command_main(args: SearchArgs) -> Result<(), anyhow::Error> {
    let mut query_args = vec!["query".to_string()]; // First arg is the command name
    query_args.push(args.pattern);

    if let Some(include) = args.include {
        query_args.push("--include".to_string());
        query_args.push(include);
    }
    if let Some(path) = args.path {
        query_args.push("--path".to_string());
        query_args.push(path);
    }
    query_args.push("--json".to_string()); // Always request JSON output

    query_command_main(&query_args).await
}
