use anyhow::Result;
use clap::Parser;
use ragit_command_bootstrap::bootstrap_index_self;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
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
            bootstrap_index_self().await?;
        }
    }

    Ok(())
}
