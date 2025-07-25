use ragit::prelude::*;
use ragit_commands::{
    add_command_main, archive_command_main, audit_command_main, build_command_main,
    cat_file_command_main, check_command_main, clone_command_main,
    extract_keywords_command_main, gc_command_main, help_command_main, ii_build_command_main,
    ii_reset_command_main, ii_status_command_main, init_command_main, ls_chunks_command_main,
    ls_files_command_main, ls_images_command_main, ls_models_command_main, ls_terms_command_main,
    merge_command_main, meta_command_main, migrate_command_main, model_command_main,
    muse_summarize_command_main, pdl_command_main, pull_command_main, push_command_main,
    qa_test_command_main, qa_tune_command_main, query_command_main, remove_command_main,
    status_command_main, summary_command_main, version_command_main,
};
use ragit_config_commands::run as config_command_main;

use async_recursion::async_recursion;

#[async_recursion(?Send)]
pub async fn run(args: Vec<String>) -> Result<(), Error> {
    let (args, pre_args): (Vec<String>, ragit_utils::cli_types::ParsedArgs) =
        ragit_cli::parse_pre_args(&args)?;

    if let Some(path) = pre_args.arg_flags.get("-C") {
        std::env::set_current_dir(path)?;
    }

    let command = args.get(1).map(|arg| arg.as_str());

    match command {
        Some("add") => add_command_main(&args, pre_args).await?,
        Some("archive") => archive_command_main(&args, pre_args).await?,
        Some("audit") => audit_command_main(&args, pre_args).await?,
        Some("build") => build_command_main(&args).await?,
        Some("cat-file") => cat_file_command_main(&args).await?,
        Some("check") => check_command_main(&args).await?,
        Some("clone") => clone_command_main(&args).await?,
        Some("config") => config_command_main(&args).await?,
        Some("extract-keywords") => extract_keywords_command_main(&args).await?,
        Some("gc") => gc_command_main(&args).await?,
        Some("help") => help_command_main(&args).await?,
        Some("ii") => ii_command_main(&args).await?,
        Some("init") => init_command_main(&args).await?,
        Some("ls") => ls_command_main(&args).await?,
        Some("merge") => merge_command_main(&args).await?,
        Some("meta") => meta_command_main(&args).await?,
        Some("migrate") => migrate_command_main(&args).await?,
        Some("model") => model_command_main(&args).await?,
        Some("muse-summarize") => muse_summarize_command_main(&args).await?,
        Some("pdl") => pdl_command_main(&args).await?,
        Some("pull") => pull_command_main(&args).await?,
        Some("push") => push_command_main(&args).await?,
        Some("qa-test") => qa_test_command_main(&args).await?,
        Some("qa-tune") => qa_tune_command_main(&args).await?,
        Some("query") => query_command_main(&args).await?,
        Some("remove") => remove_command_main(&args).await?,
        Some("status") => status_command_main(&args).await?,
        Some("summary") => summary_command_main(&args).await?,
        Some("version") => version_command_main(&args).await?,
        _ => {
            return Err(Error::CliError(ragit_utils::error::CliError::new_message(
                "Unknown command.".to_string(),
            )));
        }
    }

    Ok(())
}
