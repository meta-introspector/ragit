use clap::{Parser, Args};

#[derive(Parser, Debug, Clone)]
pub struct BootstrapArgs {
    
    #[clap(long)]
    #[arg(long, value_parser = clap::value_parser!(u64))]
    pub timeout_seconds: Option<u64>,
    #[clap(long)]
    #[arg(long, value_parser = clap::value_parser!(usize))]
    pub max_iterations: Option<usize>,
    #[clap(long)]
    #[arg(long, value_parser = clap::value_parser!(u64))]
    pub max_memory_gb: Option<u64>,
    #[clap(long)]
    #[arg(long, value_parser = clap::value_parser!(usize))]
    pub max_files_to_process: Option<usize>,
    #[clap(long)]
    #[arg(long, value_parser = clap::value_parser!(usize))]
    pub max_chunk_size: Option<usize>,
    #[clap(long)]
    #[arg(long, value_parser = clap::value_parser!(usize))]
    pub max_summary_len: Option<usize>,
    #[clap(long)]
    #[arg(long, value_parser = clap::value_parser!(usize))]
    pub min_summary_len: Option<usize>,
    #[clap(long)]
    #[arg(long, value_parser = clap::value_parser!(u128))]
    pub time_threshold_ms: Option<u128>,
    #[clap(long)]
    #[arg(long, value_parser = clap::value_parser!(u64))]
    pub memory_threshold_bytes: Option<u64>,
    #[clap(long)]
    pub disable_write_markdown: bool,
    #[clap(long)]
    pub disable_memory_config: bool,
    #[clap(long)]
    pub disable_prompt_copy: bool,
    #[clap(long)]
    pub disable_file_add: bool,
    #[clap(long)]
    pub disable_index_build: bool,
    #[clap(long)]
    pub disable_self_improvement: bool,
    #[clap(long)]
    pub disable_final_query: bool,
    #[clap(long)]
    pub disable_cleanup: bool,
}
