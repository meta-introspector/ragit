use anyhow::Result;
use clap::Parser;
use ragit_index_query::query;
use ragit_index_types::index_struct::Index;
use ragit_index_types::load_mode::LoadMode;
use ragit_memory_monitor::MemoryMonitor;
use ragit_utils::project_root::find_root;
use ragit_utils::prelude::*;
use ragit_utils::doc_utils::get_doc_content;
use ragit_types::query_turn::QueryTurn;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct QueryArgs {
    /// The query string
    pub query_string: String,
    /// Optional path to the index directory
    #[arg(long)]
    pub index_path: Option<PathBuf>,
}

pub async fn query_command_main(args: QueryArgs) -> Result<(), anyhow::Error> {
    let mut memory_monitor = MemoryMonitor::new(false, None, None);
    let index = if let Some(path) = args.index_path {
        Index::load(path, LoadMode::OnlyJson)?
    } else {
        Index::load(find_root()?, LoadMode::OnlyJson)?
    };

    let response = query(&index, &args.query_string, vec![], None, &mut memory_monitor).await?;
    println!("{}", response.get_message());

    Ok(())
}
