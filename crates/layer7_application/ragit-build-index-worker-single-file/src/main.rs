use anyhow::Result;
use clap::{Parser, Subcommand};
use std::env;
use std::fs;
use std::io::Write; // For stdin().read_line() and flush()
use std::path::PathBuf; // For PathBuf::from in bootstrap_command_main

mod bootstrap_commands;
use ragit_memory_monitor::MemoryMonitor;
use ragit_bootstrap_logic::build_index_logic::main_build_index::build_index;
use bootstrap_commands::constants::{CLEANUP_TEMP_DIR, MEMORY_USAGE_SUMMARY_HEADER};
use bootstrap_commands::setup_environment::setup_environment;
use bootstrap_commands::copy_prompts::copy_prompts;
use bootstrap_commands::add_bootstrap_files::add_bootstrap_files;
use bootstrap_commands::configure_memory_settings::configure_memory_settings;
use bootstrap_commands::export_chunks::export_chunks_main;
use bootstrap_commands::perform_self_improvement::perform_self_improvement;
use bootstrap_commands::perform_final_reflective_query::perform_final_reflective_query;

// Query related imports
use ragit_index_query::query;
use ragit_index_types::index_struct::Index;
use ragit_index_types::load_mode::LoadMode;
use ragit_utils::project_root::find_root;
use ragit_utils::doc_utils::get_doc_content;
use ragit_types::query_turn::QueryTurn;
use ragit_utils::prelude::*; // For ArgParser, ArgType, ArgCount, etc.

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Bootstrap the ragit index
    Bootstrap {
        #[clap(long)]
        verbose: bool,
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
}

async fn bootstrap_command_main(args: BootstrapArgs) -> Result<(), anyhow::Error> {
    let max_iterations = args.max_iterations;
    let max_files_to_process = args.max_files_to_process;
    let max_memory_gb = args.max_memory_gb;
    let mut memory_monitor = MemoryMonitor::new(args.verbose, args.time_threshold_ms, args.memory_threshold_bytes);

    memory_monitor.capture_and_log_snapshot("Initial");

    let (actual_root_dir, temp_dir, mut index) = setup_environment(
        max_memory_gb,
        &mut memory_monitor,
    ).await?;

    println!("Temporary directory: {:?}", temp_dir);

    memory_monitor.capture_and_log_snapshot("After setup_environment");

    if !args.disable_memory_config {
        memory_monitor.check_memory_limit(max_memory_gb, "Before configure_memory_settings")?;
        configure_memory_settings(
            args.verbose,
            &mut index,
            max_memory_gb,
            args.max_chunk_size,
            args.max_summary_len,
            args.min_summary_len,
            &mut memory_monitor,
        ).await?;
    }

    if !args.disable_prompt_copy {
        memory_monitor.check_memory_limit(max_memory_gb, "Before copy_prompts")?;
        copy_prompts(
            &actual_root_dir,
            &temp_dir,
            max_memory_gb,
            &mut memory_monitor,
        ).await?;
        ragit_index_types::index_impl::load_prompts::load_prompts_from_directory(&mut index, &temp_dir.join("prompts"))?;
    }

    if !args.disable_file_add {
        memory_monitor.check_memory_limit(max_memory_gb, "Before add_bootstrap_files")?;
        add_bootstrap_files(
            &actual_root_dir,
            &temp_dir,
            &mut index,
            max_memory_gb,
            &mut memory_monitor,
            max_files_to_process,
        ).await?;
    }

    if !args.disable_index_build {
        memory_monitor.check_memory_limit(max_memory_gb, "Before build_index")?;
        build_index(
            &temp_dir,
            &actual_root_dir,
            &mut index,
            max_iterations,
            max_memory_gb,
            &mut memory_monitor,
        ).await?;
    }

    if !args.disable_write_markdown {
        memory_monitor.check_memory_limit(max_memory_gb, "Before write_chunks_to_markdown")?;
        export_chunks_main::write_chunks_to_markdown(
            args.verbose,
            &temp_dir,
            &index,
            max_memory_gb,
            &mut memory_monitor,
            max_iterations,
        ).await?;
    } else {
        memory_monitor.verbose("bootstrap_index_self: Skipping writing chunks to markdown as requested.");
    }

    if !args.disable_self_improvement {
        memory_monitor.check_memory_limit(max_memory_gb, "Before perform_self_improvement")?;
        perform_self_improvement(
            args.verbose,
            &actual_root_dir,
            &temp_dir,
            &index,
            max_memory_gb,
            &mut memory_monitor,
        ).await?;
    }

    if !args.disable_final_query {
        memory_monitor.check_memory_limit(max_memory_gb, "Before perform_final_reflective_query")?;
        perform_final_reflective_query(
            args.verbose,
            &index,
            max_memory_gb,
            &mut memory_monitor,
        ).await?;
    }

    // Clean up the temporary directory
    if !args.disable_cleanup {
        fs::remove_dir_all(&temp_dir)?;
        memory_monitor.verbose(&format!("{:?}{:?}", CLEANUP_TEMP_DIR, temp_dir));
    } else {
        memory_monitor.verbose("bootstrap_index_self: Skipping cleanup of temporary directory as requested.");
    }

    // Print memory usage table
    memory_monitor.verbose(MEMORY_USAGE_SUMMARY_HEADER);
    memory_monitor.print_final_report();

    Ok(())
}

async fn query_command_main(args: QueryArgs) -> Result<(), anyhow::Error> {
    let index = if let Some(path) = args.kb_path {
        Index::load(PathBuf::from(path), LoadMode::OnlyJson)?
    } else {
        Index::load(find_root()?, LoadMode::OnlyJson)?
    };

    let query_str = args.query_string;
    let multi_turn = args.multi_turn;
    let json_mode = args.json;

    if multi_turn {
        let mut turns = vec![];

        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input == ".exit" {
                break;
            }

            let response = query(&index, &input, turns.clone(), None).await?;
            println!("{}", response.get_message());
            turns.push(QueryTurn {
                query: input.to_string(),
                response: ragit_types::query_turn::QueryResponse { response: response.get_message().to_string() },
            });
        }
    } else {
        let response = query(&index, &query_str, vec![], None).await?;

        if json_mode {
            println!("{}", serde_json::to_string_pretty(&response.to_json())?);
        } else {
            println!("{}", response.get_message());
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Bootstrap { verbose, timeout_seconds, max_iterations, max_memory_gb, max_files_to_process, max_chunk_size, max_summary_len, min_summary_len, time_threshold_ms, memory_threshold_bytes, disable_write_markdown, disable_memory_config, disable_prompt_copy, disable_file_add, disable_index_build, disable_self_improvement, disable_final_query, disable_cleanup } => bootstrap_command_main(BootstrapArgs { verbose, timeout_seconds, max_iterations, max_memory_gb, max_files_to_process, max_chunk_size, max_summary_len, min_summary_len, time_threshold_ms, memory_threshold_bytes, disable_write_markdown, disable_memory_config, disable_prompt_copy, disable_file_add, disable_index_build, disable_self_improvement, disable_final_query, disable_cleanup }).await,
        Commands::Query { query_string, no_pdl, multi_turn, json, kb_path } => query_command_main(QueryArgs { query_string, no_pdl, multi_turn, json, kb_path }).await,
    }
}

// Helper structs for clap args
#[derive(Parser)]
struct BootstrapArgs {
    #[clap(long)]
    verbose: bool,
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
}

#[derive(Parser)]
struct QueryArgs {
    query_string: String,
    #[clap(long)]
    no_pdl: bool,
    #[clap(long)]
    multi_turn: bool,
    #[clap(long)]
    json: bool,
    #[clap(long)]
    kb_path: Option<String>,
}
