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
            bootstrap_index_self(args.verbose, args.timeout, args.max_iterations, args.max_memory_gb).await?;
        }
    }

    Ok(())
}
