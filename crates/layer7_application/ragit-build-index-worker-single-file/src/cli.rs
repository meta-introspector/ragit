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
        /// Timeout for the bootstrap process in seconds.
        #[clap(long)]
        #[arg(long, value_parser = clap::value_parser!(u64))]
        timeout_seconds: Option<u64>,
        /// Maximum number of iterations for the self-improvement loop.
        #[clap(long)]
        #[arg(long, value_parser = clap::value_parser!(usize))]
        max_iterations: Option<usize>,
        /// Maximum memory in GB to allocate for the index.
        #[clap(long)]
        #[arg(long, value_parser = clap::value_parser!(u64))]
        max_memory_gb: Option<u64>,
        /// Maximum number of files to process during the bootstrap.
        #[clap(long)]
        #[arg(long, value_parser = clap::value_parser!(usize))]
        max_files_to_process: Option<usize>,
        /// Maximum size of a chunk in bytes.
        #[clap(long)]
        #[arg(long, value_parser = clap::value_parser!(usize))]
        max_chunk_size: Option<usize>,
        /// Maximum length of a summary.
        #[clap(long)]
        #[arg(long, value_parser = clap::value_parser!(usize))]
        max_summary_len: Option<usize>,
        /// Minimum length of a summary.
        #[clap(long)]
        #[arg(long, value_parser = clap::value_parser!(usize))]
        min_summary_len: Option<usize>,
        /// Time threshold in milliseconds for memory monitoring.
        #[clap(long)]
        #[arg(long, value_parser = clap::value_parser!(u128))]
        time_threshold_ms: Option<u128>,
        /// Memory threshold in bytes for memory monitoring.
        #[clap(long)]
        #[arg(long, value_parser = clap::value_parser!(u64))]
        memory_threshold_bytes: Option<u64>,
        /// Disable writing markdown output.
        #[clap(long)]
        disable_write_markdown: bool,
        /// Disable memory configuration.
        #[clap(long)]
        disable_memory_config: bool,
        /// Disable copying prompts.
        #[clap(long)]
        disable_prompt_copy: bool,
        /// Disable adding files to the index.
        #[clap(long)]
        disable_file_add: bool,
        /// Disable building the index.
        #[clap(long)]
        disable_index_build: bool,
        /// Disable the self-improvement loop.
        #[clap(long)]
        disable_self_improvement: bool,
        /// Disable the final reflective query.
        #[clap(long)]
        disable_final_query: bool,
        /// Disable cleanup of temporary directories.
        #[clap(long)]
        disable_cleanup: bool,
        /// Target for the bootstrap process (e.g., "all", "file_path").
        #[clap(long)]
        target: Option<String>,
        /// Enable verbose output.
        #[clap(long)]
        verbose: bool,
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
    /// Search the ragit index for file content
    Search {
        #[clap(flatten)]
        args: crate::args::search_args::SearchArgs,
    },
}