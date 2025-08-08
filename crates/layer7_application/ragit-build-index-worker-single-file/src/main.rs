use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::PathBuf; // For PathBuf::from in bootstrap_command_main

mod bootstrap_commands;
use ragit_memory_monitor::MemoryMonitor;
use ragit_bootstrap_logic::build_index_logic::main_build_index::build_index;
use bootstrap_commands::constants::{AFTER_SETUP_ENV, BEFORE_COPY_PROMPTS, BEFORE_ADD_FILES, BEFORE_BUILD_INDEX, CLEANUP_TEMP_DIR, MEMORY_USAGE_SUMMARY_HEADER};
use bootstrap_commands::setup_environment::setup_environment;
use bootstrap_commands::copy_prompts::copy_prompts;
use bootstrap_commands::add_bootstrap_files::add_bootstrap_files;
use bootstrap_commands::configure_memory_settings::configure_memory_settings;
use bootstrap_commands::export_chunks::export_chunks_main;
use bootstrap_commands::self_improvement::run_self_improvement_loop::run_self_improvement_loop;
use bootstrap_commands::perform_final_reflective_query::perform_final_reflective_query;

use ragit_index_query::query;
use ragit_index_types::index_struct::Index;
use ragit_index_types::load_mode::LoadMode;
use ragit_utils::project_root::find_root;
use ragit_types::query_turn::QueryTurn;
use std::collections::HashMap;

mod cli;
use cli::{Cli, Commands};

mod args;
use crate::args::bootstrap_args::BootstrapArgs;
use crate::args::query_args::QueryArgs;
use crate::args::top_terms_args::TopTermsArgs;


mod search_commands;
use search_commands::search_command_main;

async fn setup_bootstrap_environment(
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
) -> Result<(PathBuf, PathBuf, Index), anyhow::Error> {
    memory_monitor.verbose("Setting up environment...");
    let (actual_root_dir, temp_dir, index) = setup_environment(
        max_memory_gb,
        memory_monitor,
    ).await?;
    memory_monitor.verbose(&format!("Temporary directory: {:?}", temp_dir));
    memory_monitor.capture_and_log_snapshot(AFTER_SETUP_ENV);
    memory_monitor.verbose("Environment setup complete.");
    Ok((actual_root_dir, temp_dir, index))
}

async fn configure_memory_settings_step(
    args: &BootstrapArgs,
    index: &mut Index,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
) -> Result<(), anyhow::Error> {
    if !args.disable_memory_config {
        memory_monitor.verbose("Configuring memory settings...");
        memory_monitor.check_memory_limit(max_memory_gb, "Before configure_memory_settings")?;
        configure_memory_settings(
            index,
            max_memory_gb,
            args.max_chunk_size,
            args.max_summary_len,
            args.min_summary_len,
            memory_monitor,
        ).await?;
        memory_monitor.verbose("Memory settings configured.");
    }
    Ok(())
}

async fn copy_prompts_step(
    args: &BootstrapArgs,
    actual_root_dir: &PathBuf,
    temp_dir: &PathBuf,
    index: &mut Index,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
) -> Result<(), anyhow::Error> {
    if !args.disable_prompt_copy {
        memory_monitor.verbose("Copying prompts...");
        memory_monitor.check_memory_limit(max_memory_gb, BEFORE_COPY_PROMPTS)?;
        copy_prompts(
            actual_root_dir,
            temp_dir,
            max_memory_gb,
            memory_monitor,
        ).await?;
        ragit_index_types::index_impl::load_prompts::load_prompts_from_directory(index, &temp_dir.join("prompts"))?;
        memory_monitor.verbose("Prompts copied and loaded.");
    }
    Ok(())
}

async fn add_bootstrap_files_step(
    verbose: bool,
    actual_root_dir: &PathBuf,
    temp_dir: &PathBuf,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
    max_files_to_process: Option<usize>,
    target: Option<String>,
) -> Result<tokio::sync::mpsc::Receiver<PathBuf>, anyhow::Error> {
    memory_monitor.verbose("Adding bootstrap files...");
    memory_monitor.check_memory_limit(max_memory_gb, BEFORE_ADD_FILES)?;
    let file_receiver = add_bootstrap_files(
        actual_root_dir,
        temp_dir,
        max_memory_gb,
        memory_monitor,
        args.max_files_to_process,
        args.target,
    ).await?;
    memory_monitor.verbose("Bootstrap files added.");
    Ok(file_receiver)
}

async fn build_index_step(
    args: &BootstrapArgs,
    temp_dir: &PathBuf,
    actual_root_dir: &PathBuf,
    index: &mut Index,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
    file_receiver: tokio::sync::mpsc::Receiver<PathBuf>,
) -> Result<(), anyhow::Error> {
    if !args.disable_index_build {
        memory_monitor.verbose("Building index...");
        memory_monitor.check_memory_limit(max_memory_gb, BEFORE_BUILD_INDEX)?;
        build_index(
            temp_dir,
            actual_root_dir,
            index,
            args.max_iterations,
            max_memory_gb,
            memory_monitor,
            file_receiver,
        ).await?;
        memory_monitor.verbose("Index built.");
    }
    Ok(())
}

async fn export_chunks_step(
    args: &BootstrapArgs,
    temp_dir: &PathBuf,
    index: &mut Index,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
) -> Result<(), anyhow::Error> {
    if !args.disable_write_markdown {
        memory_monitor.verbose("Exporting chunks to markdown...");
        memory_monitor.check_memory_limit(max_memory_gb, "Before write_chunks_to_markdown")?;
        export_chunks_main::write_chunks_to_markdown(
            args.verbose,
            temp_dir,
            index,
            max_memory_gb,
            memory_monitor,
            args.max_iterations,
        ).await?;
        memory_monitor.verbose("Chunks exported.");
    } else {
        memory_monitor.verbose("bootstrap_index_self: Skipping writing chunks to markdown as requested.");
    }
    Ok(())
}

async fn save_index_step(
    index: &mut Index,
    temp_dir: &PathBuf,
    memory_monitor: &mut MemoryMonitor,
) -> Result<(), anyhow::Error> {
    memory_monitor.verbose("Saving index to file...");
    ragit_index_save_to_file::save_index_to_file(index, temp_dir.join(".ragit").join("index.json"))?;
    memory_monitor.verbose("Index saved to file.");
    Ok(())
}

async fn run_self_improvement_step(
    args: &BootstrapArgs,
    actual_root_dir: &PathBuf,
    temp_dir: &PathBuf,
    index: &mut Index,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
) -> Result<(), anyhow::Error> {
    if !args.disable_self_improvement {
        memory_monitor.verbose("Running self-improvement loop...");
        memory_monitor.check_memory_limit(max_memory_gb, "Before perform_self_improvement")?;
        run_self_improvement_loop(
            args.verbose,
            actual_root_dir,
            temp_dir,
            index,
            max_memory_gb,
            memory_monitor,
            args.max_iterations,
        ).await?;
        memory_monitor.verbose("Self-improvement loop finished.");
    }
    Ok(())
}

async fn perform_final_query_step(
    args: &BootstrapArgs,
    index: &mut Index,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
) -> Result<(), anyhow::Error> {
    if !args.disable_final_query {
        memory_monitor.verbose("Performing final reflective query...");
        memory_monitor.check_memory_limit(max_memory_gb, "Before perform_final_reflective_query")?;
        perform_final_reflective_query(
            args.verbose,
            index,
            max_memory_gb,
            memory_monitor,
        ).await?;
        memory_monitor.verbose("Final reflective query performed.");
    }
    Ok(())
}

async fn generate_keyword_report_step(
    args: &BootstrapArgs,
    memory_monitor: &mut MemoryMonitor,
) -> Result<(), anyhow::Error> {
    memory_monitor.verbose("Generating keyword report...");
    let term_frequencies = top_terms_command_main(TopTermsArgs { count: Some(10), kb_path: None }, memory_monitor).await?;
    memory_monitor.verbose(&format!("Top 10 keywords: {:?}", term_frequencies.keys().take(10).collect::<Vec<_>>()));
    memory_monitor.verbose("Keyword report generated.");
    Ok(())
}

async fn cleanup_temp_dir_step(
    args: &BootstrapArgs,
    temp_dir: &PathBuf,
    memory_monitor: &mut MemoryMonitor,
) -> Result<(), anyhow::Error> {
    if !args.disable_cleanup {
        memory_monitor.verbose("Cleaning up temporary directory...");
        fs::remove_dir_all(temp_dir)?;
        memory_monitor.verbose(&format!("{:?}{:?}", CLEANUP_TEMP_DIR, temp_dir));
        memory_monitor.verbose("Temporary directory cleaned up.");
    } else {
        memory_monitor.verbose("bootstrap_index_self: Skipping cleanup of temporary directory as requested.");
    }
    Ok(())
}

async fn print_final_report_step(
    memory_monitor: &mut MemoryMonitor,
) -> Result<(), anyhow::Error> {
    memory_monitor.verbose(MEMORY_USAGE_SUMMARY_HEADER);
    memory_monitor.print_final_report();
    Ok(())
}

//use anyhow::Result;
//use clap::Parser;
//use std::fs;
//use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;

//mod bootstrap_commands;
//use ragit_memory_monitor::MemoryMonitor;
//use ragit_bootstrap_logic::build_index_logic::main_build_index::build_index;
////use bootstrap_commands::constants::{AFTER_SETUP_ENV, BEFORE_COPY_PROMPTS, BEFORE_ADD_FILES, BEFORE_BUILD_INDEX, CLEANUP_TEMP_DIR, MEMORY_USAGE_SUMMARY_HEADER};
//use bootstrap_commands::setup_environment::setup_environment;
//use bootstrap_commands::copy_prompts::copy_prompts;
//use bootstrap_commands::add_bootstrap_files::add_bootstrap_files;
//use bootstrap_commands::configure_memory_settings::configure_memory_settings;
//use bootstrap_commands::export_chunks::export_chunks_main;
//use bootstrap_commands::self_improvement::run_self_improvement_loop::run_self_improvement_loop;
//use bootstrap_commands::perform_final_reflective_query::perform_final_reflective_query;

//use ragit_index_query::query;
//use ragit_index_types::index_struct::Index;
//use ragit_index_types::load_mode::LoadMode;
//use ragit_utils::project_root::find_root;
//use ragit_types::query_turn::QueryTurn;
//use std::collections::HashMap;
use tokio::sync::mpsc;

//mod cli;
//use cli::{Cli, Commands};

//mod args;
//use crate::args::bootstrap_args::BootstrapArgs;
//use crate::args::query_args::QueryArgs;
//use crate::args::top_terms_args::TopTermsArgs;


//mod search_commands;
//use search_commands::search_command_main;

pub struct BootstrapContext {
    pub args: BootstrapArgs,
    pub actual_root_dir: PathBuf,
    pub temp_dir: PathBuf,
    pub index: Index,
    pub memory_monitor: Arc<Mutex<MemoryMonitor>>,
    pub file_receiver: Option<mpsc::Receiver<PathBuf>>,
    pub seen_keywords: Arc<Mutex<HashSet<String>>>,
}


// async fn configure_memory_settings_step(
//     context: &mut BootstrapContext,
// ) -> Result<(), anyhow::Error> {
//     if !context.args.disable_memory_config {
//         context.memory_monitor.lock().unwrap().verbose("Configuring memory settings...");
//         context.memory_monitor.lock().unwrap().check_memory_limit(context.args.max_memory_gb, "Before configure_memory_settings")?;
//         configure_memory_settings(
//             &mut context.index,
//             context.args.max_memory_gb,
//             context.args.max_chunk_size,
//             context.args.max_summary_len,
//             context.args.min_summary_len,
//             &mut context.memory_monitor.lock().unwrap(),
//         ).await?;
//         context.memory_monitor.lock().unwrap().verbose("Memory settings configured.");
//     }
//     Ok(())
// }

// async fn copy_prompts_step(
//     context: &mut BootstrapContext,
// ) -> Result<(), anyhow::Error> {
//     if !context.args.disable_prompt_copy {
//         context.memory_monitor.lock().unwrap().verbose("Copying prompts...");
//         context.memory_monitor.lock().unwrap().check_memory_limit(context.args.max_memory_gb, BEFORE_COPY_PROMPTS)?;
//         copy_prompts(
//             &context.actual_root_dir,
//             &context.temp_dir,
//             context.args.max_memory_gb,
//             &mut context.memory_monitor.lock().unwrap(),
//         ).await?;
//         ragit_index_types::index_impl::load_prompts::load_prompts_from_directory(&mut context.index, &context.temp_dir.join("prompts"))?;
//         context.memory_monitor.lock().unwrap().verbose("Prompts copied and loaded.");
//     }
//     Ok(())
// }

// async fn add_bootstrap_files_step(
//     context: &mut BootstrapContext,
// ) -> Result<(), anyhow::Error> {
//     if !context.args.disable_file_add {
//         context.memory_monitor.lock().unwrap().verbose("Adding bootstrap files...");
//         context.memory_monitor.lock().unwrap().check_memory_limit(context.args.max_memory_gb, BEFORE_ADD_FILES)?;
//         let file_receiver = add_bootstrap_files(
//             context.args.verbose,
//             &context.actual_root_dir,
//             &context.temp_dir,
//             context.args.max_memory_gb,
//             context.memory_monitor.clone(),
//             context.args.max_files_to_process,
//             context.args.target.clone(),
//         ).await?;
//         context.file_receiver = Some(file_receiver);
//         context.memory_monitor.lock().unwrap().verbose("Bootstrap files added.");
//     } else {
//         context.memory_monitor.lock().unwrap().verbose("bootstrap_index_self: Skipping file add as requested.");
//         // Return a closed channel if file add is disabled
//         let (_, rx) = tokio::sync::mpsc::channel(1);
//         context.file_receiver = Some(rx);
//     }
//     Ok(())
// }

// async fn build_index_step(
//     context: &mut BootstrapContext,
// ) -> Result<(), anyhow::Error> {
//     if !context.args.disable_index_build {
//         context.memory_monitor.lock().unwrap().verbose("Building index...");
//         context.memory_monitor.lock().unwrap().check_memory_limit(context.args.max_memory_gb, BEFORE_BUILD_INDEX)?;
//         build_index(
//             &context.temp_dir,
//             &context.actual_root_dir,
//             &mut context.index,
//             context.args.max_iterations,
//             context.args.max_memory_gb,
//             &mut context.memory_monitor.lock().unwrap(),
//             context.file_receiver.take().unwrap(), // Take ownership of the receiver
//             context.seen_keywords.clone(),
//         ).await?;
//         context.memory_monitor.lock().unwrap().verbose("Index built.");
//     }
//     Ok(())
// }

// async fn export_chunks_step(
//     context: &mut BootstrapContext,
// ) -> Result<(), anyhow::Error> {
//     if !context.args.disable_write_markdown {
//         context.memory_monitor.lock().unwrap().verbose("Exporting chunks to markdown...");
//         context.memory_monitor.lock().unwrap().check_memory_limit(context.args.max_memory_gb, "Before write_chunks_to_markdown")?;
//         export_chunks_main::write_chunks_to_markdown(
//             context.args.verbose,
//             &context.temp_dir,
//             &context.index,
//             context.args.max_memory_gb,
//             &mut context.memory_monitor.lock().unwrap(),
//             context.args.max_iterations,
//         ).await?;
//         context.memory_monitor.lock().unwrap().verbose("Chunks exported.");
//     } else {
//         context.memory_monitor.lock().unwrap().verbose("bootstrap_index_self: Skipping writing chunks to markdown as requested.");
//     }
//     Ok(())
// }

// async fn save_index_step(
//     context: &mut BootstrapContext,
// ) -> Result<(), anyhow::Error> {
//     context.memory_monitor.lock().unwrap().verbose("Saving index to file...");
//     ragit_index_save_to_file::save_index_to_file(&context.index, context.temp_dir.join(".ragit").join("index.json"))?;
//     context.memory_monitor.lock().unwrap().verbose("Index saved to file.");
//     Ok(())
// }

// async fn run_self_improvement_step(
//     context: &mut BootstrapContext,
// ) -> Result<(), anyhow::Error> {
//     if !context.args.disable_self_improvement {
//         context.memory_monitor.lock().unwrap().verbose("Running self-improvement loop...");
//         context.memory_monitor.lock().unwrap().check_memory_limit(context.args.max_memory_gb, "Before perform_self_improvement")?;
//         run_self_improvement_loop(
//             context.args.verbose,
//             &context.actual_root_dir,
//             &context.temp_dir,
//             &context.index,
//             context.args.max_memory_gb,
//             &mut context.memory_monitor.lock().unwrap(),
//             context.args.max_iterations,
//         ).await?;
//         context.memory_monitor.lock().unwrap().verbose("Self-improvement loop finished.");
//     }
//     Ok(())
// }

// async fn perform_final_query_step(
//     context: &mut BootstrapContext,
// ) -> Result<(), anyhow::Error> {
//     if !context.args.disable_final_query {
//         context.memory_monitor.lock().unwrap().verbose("Performing final reflective query...");
//         context.memory_monitor.lock().unwrap().check_memory_limit(context.args.max_memory_gb, "Before perform_final_reflective_query")?;
//         perform_final_reflective_query(
//             context.args.verbose,
//             &context.index,
//             context.args.max_memory_gb,
//             &mut context.memory_monitor.lock().unwrap(),
//         ).await?;
//         context.memory_monitor.lock().unwrap().verbose("Final reflective query performed.");
//     }
//     Ok(())
// }

// async fn generate_keyword_report_step(
//     context: &mut BootstrapContext,
// ) -> Result<(), anyhow::Error> {
//     context.memory_monitor.lock().unwrap().verbose("Generating keyword report...");
//     let term_frequencies = top_terms_command_main(TopTermsArgs { count: Some(10), kb_path: None }, &mut context.memory_monitor.lock().unwrap()).await?;
//     context.memory_monitor.lock().unwrap().verbose(&format!("Top 10 keywords: {:?}", term_frequencies.keys().take(10).collect::<Vec<_>>()));
//     context.memory_monitor.lock().unwrap().verbose("Keyword report generated.");
//     Ok(())
// }

// async fn cleanup_temp_dir_step(
//     context: &mut BootstrapContext,
// ) -> Result<(), anyhow::Error> {
//     if !context.args.disable_cleanup {
//         context.memory_monitor.lock().unwrap().verbose("Cleaning up temporary directory...");
//         fs::remove_dir_all(&context.temp_dir)?;
//         context.memory_monitor.lock().unwrap().verbose(&format!("{:?}{:?}", CLEANUP_TEMP_DIR, context.temp_dir));
//         context.memory_monitor.lock().unwrap().verbose("Temporary directory cleaned up.");
//     } else {
//         context.memory_monitor.lock().unwrap().verbose("bootstrap_index_self: Skipping cleanup of temporary directory as requested.");
//     }
//     Ok(())
// }

// async fn print_final_report_step(
//     context: &mut BootstrapContext,
// ) -> Result<(), anyhow::Error> {
//     context.memory_monitor.lock().unwrap().verbose(MEMORY_USAGE_SUMMARY_HEADER);
//     context.memory_monitor.lock().unwrap().print_final_report();
//     Ok(())
// }

async fn bootstrap_command_main(args: BootstrapArgs) -> Result<(), anyhow::Error> {
    let mut memory_monitor = MemoryMonitor::new(args.verbose, args.time_threshold_ms, args.memory_threshold_bytes);
    memory_monitor.verbose("Starting bootstrap process.");
    memory_monitor.capture_and_log_snapshot("Initial");

    let mut context = BootstrapContext {
        args,
        actual_root_dir: PathBuf::new(), // Placeholder, will be filled by setup_bootstrap_environment
        temp_dir: PathBuf::new(), // Placeholder
        index: Index::default(), // Placeholder
        memory_monitor: Arc::new(Mutex::new(memory_monitor)),
        file_receiver: None,
        seen_keywords: Arc::new(Mutex::new(HashSet::new())),
    };

    setup_bootstrap_environment(&mut context).await?;
    configure_memory_settings_step(&mut context).await?;
    copy_prompts_step(&mut context).await?;
    add_bootstrap_files_step(&mut context).await?;
    build_index_step(&mut context).await?;
    export_chunks_step(&mut context).await?;
    save_index_step(&mut context).await?;
    run_self_improvement_step(&mut context).await?;
    perform_final_query_step(&mut context).await?;
    generate_keyword_report_step(&mut context).await?;
    cleanup_temp_dir_step(&mut context).await?;
    print_final_report_step(&mut context).await?;

    Ok(())
}

async fn query_command_main(args: QueryArgs, memory_monitor: &mut MemoryMonitor) -> Result<(), anyhow::Error> {
    memory_monitor.verbose("query_command_main: Starting query command.");
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
            memory_monitor.verbose(&format!("query_command_main: Executing query: {}", input));
            let response = query(&index, &input, turns.clone(), None, memory_monitor).await?;
            memory_monitor.verbose("query_command_main: Query executed.");
            println!("{}", response.get_message());
            turns.push(QueryTurn {
                query: input.to_string(),
                response: ragit_types::query_turn::QueryResponse { response: response.get_message().to_string() },
            });
        }
    } else {
        memory_monitor.verbose(&format!("query_command_main: Executing query: {}", query_str));
        let response = query(&index, &query_str, vec![], None, memory_monitor).await?;
        println!("query_command_main: Query executed.");

        if json_mode {
            println!("{}", serde_json::to_string_pretty(&response.to_json())?);
        } else {
            println!("{}", response.get_message());
        }
    }

    Ok(())
}

async fn top_terms_command_main(args: TopTermsArgs, _memory_monitor: &mut MemoryMonitor) -> Result<HashMap<String, usize>, anyhow::Error> {
    use std::collections::HashMap;

    let index = if let Some(path) = args.kb_path {
        Index::load(PathBuf::from(path), LoadMode::OnlyJson)?
    } else {
        Index::load(find_root()?, LoadMode::OnlyJson)?
    };

    let mut term_frequencies: HashMap<String, usize> = HashMap::new();

    for chunk in index.get_chunks() {
        let content = chunk.data.as_str();
        for word in content.split_whitespace() {
            let cleaned_word = word.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric()).to_string();
            if !cleaned_word.is_empty() {
                *term_frequencies.entry(cleaned_word).or_insert(0) += 1;
            }
        }
    }

    let mut sorted_terms: Vec<(&String, &usize)> = term_frequencies.iter().collect();
    sorted_terms.sort_by(|a, b| b.1.cmp(a.1));

    let count = args.count.unwrap_or(10);
    println!("\nTop {} terms in the index:", count);
    println!("----------------------------------");
    for (term, freq) in sorted_terms.iter().take(count) {
        println!("{}: {}", term, freq);
    }
    println!("----------------------------------");

    Ok(term_frequencies)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut memory_monitor = MemoryMonitor::new(true, None, None);

    match cli.command {
        Commands::Bootstrap { verbose, timeout_seconds, max_iterations, max_memory_gb, max_files_to_process, max_chunk_size, max_summary_len, min_summary_len, time_threshold_ms, memory_threshold_bytes, disable_write_markdown, disable_memory_config, disable_prompt_copy, disable_file_add, disable_index_build, disable_self_improvement, disable_final_query, disable_cleanup, target } => {
            let mut memory_monitor = MemoryMonitor::new(verbose, time_threshold_ms, memory_threshold_bytes);
            bootstrap_command_main(BootstrapArgs {
		verbose,
		timeout_seconds, max_iterations, max_memory_gb, max_files_to_process, max_chunk_size, max_summary_len, min_summary_len, time_threshold_ms, memory_threshold_bytes, disable_write_markdown, disable_memory_config, disable_prompt_copy, disable_file_add, disable_index_build, disable_self_improvement, disable_final_query, disable_cleanup, target }, &mut memory_monitor).await
        },
        Commands::Query { query_string, no_pdl, multi_turn, json, kb_path } => {
            query_command_main(QueryArgs { query_string, no_pdl, multi_turn, json, kb_path,
					   //verbose: true
	    }, &mut memory_monitor).await
        },
        Commands::TopTerms { count, kb_path } => {
            top_terms_command_main(TopTermsArgs { count, kb_path }, &mut memory_monitor).await
        },
        Commands::Search { args } => {
            search_command_main(args).await
        }
    }
}
