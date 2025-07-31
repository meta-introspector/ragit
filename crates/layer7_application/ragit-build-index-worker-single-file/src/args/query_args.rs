use clap::{Parser, Args};

#[derive(Parser, Debug, Clone)]
pub struct QueryArgs {
    pub query_string: String,
    #[clap(long)]
    pub no_pdl: bool,
    #[clap(long)]
    pub multi_turn: bool,
    #[clap(long)]
    pub json: bool,
    #[clap(long)]
    pub kb_path: Option<String>,
    // #[clap(long)]
    // pub verbose: bool,
}
