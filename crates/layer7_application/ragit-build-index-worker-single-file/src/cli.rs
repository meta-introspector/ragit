use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Bootstrap the ragit index
    Bootstrap {
        #[clap(long)]
        #[arg(long, value_parser = clap::value_parser!(u64))]
        timeout_seconds: Option<u64>,
        #[clap(long)]
        #[arg(long, value_parser = clap::value_parser!(usize))]
        max_iterations: Option<usize>,
        #[clap(long)]
        #[arg(long, value_parser = clap::value_parser!(u64))]
        max_memory_gb: Option<u64>,
        #[clap(long)]
        #[arg(long, value_parser = clap::value_parser!(usize))]
        max_files_to_process: Option<usize>,
        #[clap(long)]
        #[arg(long, value_parser = clap::value_parser!(usize))]
        max_chunk_size: Option<usize>,
        #[clap(long)]
        #[arg(long, value_parser = clap::value_parser!(usize))]
        max_summary_len: Option<usize>,
        #[clap(long)]
        #[arg(long, value_parser = clap::value_parser!(usize))]
        min_summary_len: Option<usize>,
        #[clap(long)]
        #[arg(long, value_parser = clap::value_parser!(u128))]
        time_threshold_ms: Option<u128>,
        #[clap(long)]
        #[arg(long, value_parser = clap::value_parser!(u64))]
        memory_threshold_bytes: Option<u64>,
        #[clap(long)]
        disable_write_markdown: bool,
        #[clap(long)]
        disable_memory_config: bool,
        #[clap(long)]
        disable_prompt_copy: bool,
        #[clap(long)]
        disable_file_add: bool,
        #[clap(long)]
        disable_index_build: bool,
        #[clap(long)]
        disable_self_improvement: bool,
        #[clap(long)]
        disable_final_query: bool,
        #[clap(long)]
        disable_cleanup: bool,
    },
    /// Query the ragit index
    Query {
        query_string: String,
        #[clap(long)]
        no_pdl: bool,
        #[clap(long)]
        multi_turn: bool,
        #[clap(long)]
        json: bool,
        #[clap(long)]
        kb_path: Option<String>,
    },
    /// Report top terms in the index
    TopTerms {
        #[clap(long)]
        count: Option<usize>,
        #[clap(long)]
        kb_path: Option<String>,
    },
}