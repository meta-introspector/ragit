use ragit::prelude::*;
use ragit_commands::{
    help_command_main, meta_command_main, migrate_command_main, query_command_main, remove_command_main,
    status_command_main, summary_command_main,
};
use ragit_command_add::add_command_main;
use ragit_command_archive::archive_command_main;
use ragit_command_audit::audit_command_main;
use ragit_command_build::build_command_main;
use ragit_command_cat_file::cat_file_command_main;
use ragit_command_check::check_command_main;
use ragit_command_clone::clone_command_main;
use ragit_command_extract_keywords::extract_keywords_command_main;
use ragit_command_gc::gc_command_main;
use ragit_command_ii_build::ii_build_command_main;
use ragit_command_ii_reset::ii_reset_command_main;
use ragit_command_ii_status::ii_status_command_main;
use ragit_command_init::init_command_main;
use ragit_command_ls::ls_command_main;
use ragit_command_merge::merge_command_main;
use ragit_command_model::model_command_main;
use ragit_command_muse_summarize::muse_summarize_command_main;
use ragit_command_pdl::pdl_command_main;
use ragit_command_pull::pull_command_main;
use ragit_command_push::push_command_main;
// use ragit_command_qa_test::qa_test_command_main;
// use ragit_command_qa_tune::qa_tune_command_main;
use ragit_command_version::version_command_main;
use ragit_config_commands::run as config_command_main;
use ragit_command_bootstrap::bootstrap_command_main;

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
        Some("ii") => {
            let subcommand = args.get(2).map(|arg| arg.as_str());
            match subcommand {
                Some("build") => ii_build_command_main(&args).await?,
                Some("reset") => ii_reset_command_main(&args).await?,
                Some("status") => ii_status_command_main(&args).await?,
                _ => {
                    return Err(Error::CliError(ragit_utils::error::CliError::new_message(
                        "Unknown ii subcommand. Use 'rag ii help' for more information.".to_string(),
                    )));
                }
            }
        },
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
        // Some("qa-test") => qa_test_command_main(&args).await?,
        // Some("qa-tune") => qa_tune_command_main(&args).await?,
        Some("query") => query_command_main(&args).await?,
        Some("remove") => remove_command_main(&args).await?,
        Some("status") => status_command_main(&args).await?,
        Some("summary") => summary_command_main(&args).await?,
        Some("version") => version_command_main(&args).await?,
        Some("bootstrap") => bootstrap_command_main(&args).await?,
        _ => {
            return Err(Error::CliError(ragit_utils::error::CliError::new_message(
                "Unknown command.".to_string(),
            )));
        }
    }

    Ok(())
}
