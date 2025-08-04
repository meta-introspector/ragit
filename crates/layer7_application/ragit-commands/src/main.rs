use anyhow::Result;
use clap::Parser;
use std::process::Command;
use std::path::PathBuf;

// #[global_allocator]
// static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
use ragit_command_bootstrap::bootstrap_index_self;
use ragit_commands::commands::query::{query_command_main, QueryArgs};
use ragit_command_duplicate_chunks::{duplicate_chunks_command_main, DuplicateChunksArgs};
use ragit_dyim::DyimCommand;



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,

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

    /// Maximum chunk size for content splitting
    #[arg(long, global = true)]
    max_chunk_size: Option<usize>,

    /// Maximum summary length for generated summaries
    #[arg(long, global = true)]
    max_summary_len: Option<usize>,

    /// Minimum summary length for generated summaries
    #[arg(long, global = true)]
    min_summary_len: Option<usize>,

    /// Time threshold in milliseconds for performance alerts
    #[arg(long, global = true)]
    time_threshold_ms: Option<u128>,

    /// Memory threshold in bytes for memory alerts
    #[arg(long, global = true)]
    memory_threshold_bytes: Option<u64>,

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

    /// Disable cleanup of temporary directory
    #[arg(long, global = true)]
    disable_cleanup: bool,

    /// Target specific directories for indexing (all, submodules, crates, src, docs)
    #[arg(long, global = true)]
    target: Option<String>,

    /// Path to the index directory
    #[arg(long, global = true)]
    index_path: Option<PathBuf>,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Run the bootstrap process
    Bootstrap,
    /// Run the new bootstrap process (fixed-size chunking)
    BootstrapNew,
    /// Index the current project
    Index,
    /// Query the ragit index
    Query(QueryArgs),
    /// Find and list duplicate chunks
    DuplicateChunks(DuplicateChunksArgs),
    
    /// Request a new change
    #[clap(external_subcommand)]
    External(Vec<String>),
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("Verbose mode enabled.");

    match args.command {
        Commands::Bootstrap => {
            bootstrap_index_self(
                true,
                args.timeout,
                args.max_iterations,
                args.max_memory_gb,
                args.max_files_to_process,
                args.max_chunk_size,
                args.max_summary_len,
                args.min_summary_len,
                args.time_threshold_ms,
                args.memory_threshold_bytes,
                args.disable_write_markdown,
                args.disable_memory_config,
                args.disable_prompt_copy,
                args.disable_file_add,
                args.disable_index_build,
                args.disable_self_improvement,
                args.disable_final_query,
                args.disable_cleanup,
                Some("all".to_string()),
            ).await?;
        },
        Commands::Index => {
            bootstrap_index_self(
                true,
                args.timeout,
                args.max_iterations,
                args.max_memory_gb,
                args.max_files_to_process,
                args.max_chunk_size,
                args.max_summary_len,
                args.min_summary_len,
                args.time_threshold_ms,
                args.memory_threshold_bytes,
                args.disable_write_markdown,
                args.disable_memory_config,
                args.disable_prompt_copy,
                args.disable_file_add,
                args.disable_index_build,
                args.disable_self_improvement,
                args.disable_final_query,
                args.disable_cleanup,
                Some("all".to_string()),
            ).await?;
        },
        Commands::BootstrapNew => {
            let mut cmd = Command::new("cargo");
            cmd.arg("run");
            cmd.arg("--package");
            cmd.arg("ragit-build-index-worker-single-file");
            cmd.arg("--"); // Pass arguments to the binary

            cmd.arg("bootstrap");

            // Then subcommand-specific flags
            if let Some(timeout) = args.timeout {
                cmd.arg("--timeout-seconds").arg(timeout.to_string());
            }
            if let Some(max_iterations) = args.max_iterations {
                cmd.arg("--max-iterations").arg(max_iterations.to_string());
            }
            if let Some(max_memory_gb) = args.max_memory_gb {
                cmd.arg("--max-memory-gb").arg(max_memory_gb.to_string());
            }
            if let Some(max_files_to_process) = args.max_files_to_process {
                cmd.arg("--max-files-to-process").arg(max_files_to_process.to_string());
            }
            if args.disable_write_markdown {
                cmd.arg("--disable-write-markdown");
            }
            if args.disable_memory_config {
                cmd.arg("--disable-memory-config");
            }
            if args.disable_prompt_copy {
                cmd.arg("--disable-prompt-copy");
            }
            if args.disable_file_add {
                cmd.arg("--disable-file-add");
            }
            if args.disable_index_build {
                cmd.arg("--disable-index-build");
            }
            if args.disable_self_improvement {
                cmd.arg("--disable-self-improvement");
            }
            if args.disable_final_query {
                cmd.arg("--disable-final-query");
            }
            if args.disable_cleanup {
                cmd.arg("--disable-cleanup");
            }

            let status = cmd.status()?;
            if !status.success() {
                anyhow::bail!("cargo run --package ragit-build-index-worker-single-file failed with status: {}", status);
            }
        },
        Commands::Query(mut query_args) => {
            query_args.index_path = Some(PathBuf::from("/data/data/com.termux/files/home/storage/github/ragit/tmp_bootstrap"));
            query_command_main(query_args).await?;
        },
        Commands::DuplicateChunks(args) => {
            duplicate_chunks_command_main(args).await?;
        },
        Commands::External(ext_args) => {
            let subcommand = &ext_args[0];
            let subcommand_args = &ext_args[1..];

            let mut cmd = Command::new("cargo");
            cmd.arg("run");
            cmd.arg("--package");

            match subcommand.as_str() {
                "change-request" => {
                    cmd.arg("ragit-command-change-request");
                    cmd.arg("--"); // Pass arguments to the binary
                    cmd.args(subcommand_args);

                    let status = cmd.status()?;
                    if !status.success() {
                        anyhow::bail!("Subcommand {} failed with status: {}", subcommand, status);
                    }
                },
                "dyim" => {
                    let dyim_cmd = DyimCommand::Dyim(subcommand_args.to_vec());
                    dyim_cmd.run().await?;
                }
                _ => {
                    anyhow::bail!("Unknown subcommand: {}", subcommand);
                }
            }
        }
    }

    Ok(())
}