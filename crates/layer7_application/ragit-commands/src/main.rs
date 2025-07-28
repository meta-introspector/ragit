use anyhow::Result;
use clap::Parser;

// #[global_allocator]
// static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
use ragit_command_bootstrap::bootstrap_index_self;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Timeout for the bootstrap process in seconds
    #[arg(long, global = true)]
    timeout: Option<u64>,

    /// Max iterations for debugging the build dashboard
    #[arg(long, global = true)]
    max_iterations: Option<usize>,

    /// Maximum memory in GB for the bootstrap process
    #[arg(long, global = true)]
    max_memory_gb: Option<u64>,

    /// Maximum number of files to process during bootstrap
    #[arg(long, global = true)]
    max_files_to_process: Option<usize>,

    /// Disable writing chunks to markdown file
    #[arg(long, global = true)]
    disable_write_markdown: bool,

    /// Disable memory configuration
    #[arg(long, global = true)]
    disable_memory_config: bool,

    /// Disable prompt copying
    #[arg(long, global = true)]
    disable_prompt_copy: bool,

    /// Disable file adding
    #[arg(long, global = true)]
    disable_file_add: bool,

    /// Disable index building
    #[arg(long, global = true)]
    disable_index_build: bool,

    /// Disable self-improvement
    #[arg(long, global = true)]
    disable_self_improvement: bool,

    /// Disable final reflective query
    #[arg(long, global = true)]
    disable_final_query: bool,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Run the bootstrap process
    Bootstrap,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("Verbose mode enabled.");
    }

    match args.command {
        Commands::Bootstrap => {
            bootstrap_index_self(
                args.verbose,
                args.timeout,
                args.max_iterations,
                args.max_memory_gb,
                args.max_files_to_process,
                args.disable_write_markdown,
                args.disable_memory_config,
                args.disable_prompt_copy,
                args.disable_file_add,
                args.disable_index_build,
                args.disable_self_improvement,
                args.disable_final_query,
            ).await?;
        }
    }

    Ok(())
}
