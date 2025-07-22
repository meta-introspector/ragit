use crate::prelude::*;
use crate::main::commands::add::add_command_main;
use crate::main::commands::archive::archive_command_main;
use crate::main::commands::audit::audit_command_main;
use crate::main::commands::build::build_command_main;
use crate::main::commands::cat_file::cat_file_command_main;
use crate::main::commands::check::check_command_main;
use crate::main::commands::clone::clone_command_main;
use crate::main::commands::config::config_command_main;
use crate::main::commands::extract_keywords::extract_keywords_command_main;
use crate::main::commands::gc::gc_command_main;
use crate::main::commands::help::help_command_main;
use crate::main::commands::ii_build::ii_command_main;
use crate::main::commands::init::init_command_main;
use crate::main::commands::ls::ls_command_main;
use crate::main::commands::merge::merge_command_main;
use crate::main::commands::meta::meta_command_main;
use crate::main::commands::migrate::migrate_command_main;
use crate::main::commands::model::model_command_main;
use crate::main::commands::muse_summarize::muse_summarize_command_main;
use crate::main::commands::pdl::pdl_command_main;
use crate::main::commands::pull::pull_command_main;
use crate::main::commands::push::push_command_main;
use crate::main::commands::qa_test::qa_test_command_main;
use crate::main::commands::qa_tune::qa_tune_command_main;
use crate::main::commands::query::query_command_main;
use crate::main::commands::remove::remove_command_main;
use crate::main::commands::status::status_command_main;
use crate::main::commands::summary::summary_command_main;
use crate::main::commands::version::version_command_main;


use async_recursion::async_recursion;

#[async_recursion(?Send)]
pub async fn run(args: Vec<String>) -> Result<(), Error> {
    let (args, pre_args): (Vec<String>, ragit_utils::cli_types::ParsedArgs) = ragit_cli::parse_pre_args(&args)?;

    if let Some(path) = pre_args.arg_flags.get("-C") {
        std::env::set_current_dir(path)?;
    }

    let command = args.get(1).map(|arg| arg.as_str());

    match command {
        Some("add") => add_command_main(args, pre_args).await?,
        Some("archive") => archive_command_main(args, pre_args).await?,
        Some("audit") => audit_command_main(args, pre_args).await?,
        Some("build") => build_command_main(args, pre_args).await?,
        Some("cat-file") => cat_file_command_main(args, pre_args).await?,
        Some("check") => check_command_main(args, pre_args).await?,
        Some("clone") => clone_command_main(args, pre_args).await?,
        Some("config") => config_command_main(args, pre_args).await?,
        Some("extract-keywords") => extract_keywords_command_main(args, pre_args).await?,
        Some("gc") => gc_command_main(args, pre_args).await?,
        Some("help") => help_command_main(args, pre_args).await?,
        Some("ii") => ii_command_main(args, pre_args).await?,
        Some("init") => init_command_main(args, pre_args).await?,
        Some("ls") => ls_command_main(args, pre_args).await?,
        Some("merge") => merge_command_main(args, pre_args).await?,
        Some("meta") => meta_command_main(args, pre_args).await?,
        Some("migrate") => migrate_command_main(args, pre_args).await?,
        Some("model") => model_command_main(args, pre_args).await?,
        Some("muse-summarize") => muse_summarize_command_main(args, pre_args).await?,
        Some("pdl") => pdl_command_main(args, pre_args).await?,
        Some("pull") => pull_command_main(args, pre_args).await?,
        Some("push") => push_command_main(args, pre_args).await?,
        Some("qa-test") => qa_test_command_main(args, pre_args).await?,
        Some("qa-tune") => qa_tune_command_main(args, pre_args).await?,
        Some("query") => query_command_main(args, pre_args).await?,
        Some("remove") => remove_command_main(args, pre_args).await?,
        Some("status") => status_command_main(args, pre_args).await?,
        Some("summary") => summary_command_main(args, pre_args).await?,
        Some("version") => version_command_main(args, pre_args).await?,
        _ => {
            return Err(Error::CliError(ragit_utils::error::CliError::new_message("Unknown command.".to_string())));
        }
    }

    Ok(())
}