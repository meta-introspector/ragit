use ragit::prelude::*;
use ragit_commands::add::add_command_main;
use ragit_commands::archive::archive_command_main;
use ragit_commands::audit::audit_command_main;
use ragit_commands::build::build_command_main;
use ragit_commands::cat_file::cat_file_command_main;
use ragit_commands::check::check_command_main;
use ragit_commands::clone::clone_command_main;
use ragit_commands::config::config_command_main;
use ragit_commands::extract_keywords::extract_keywords_command_main;
use ragit_commands::gc::gc_command_main;
use ragit_commands::help::help_command_main;
use ragit_commands::ii_build::ii_command_main;
use ragit_commands::init::init_command_main;
use ragit_commands::ls::ls_command_main;
use ragit_commands::merge::merge_command_main;
use ragit_commands::meta::meta_command_main;
use ragit_commands::migrate::migrate_command_main;
use ragit_commands::model::model_command_main;
use ragit_commands::muse_summarize::muse_summarize_command_main;
use ragit_commands::pdl::pdl_command_main;
use ragit_commands::pull::pull_command_main;
use ragit_commands::push::push_command_main;
use ragit_commands::qa_test::qa_test_command_main;
use ragit_commands::qa_tune::qa_tune_command_main;
use ragit_commands::query::query_command_main;
use ragit_commands::remove::remove_command_main;
use ragit_commands::status::status_command_main;
use ragit_commands::summary::summary_command_main;
use ragit_commands::version::version_command_main;


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
