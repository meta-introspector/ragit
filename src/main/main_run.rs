use crate::prelude::*;
use ragit_utils::index::commands::add::add_command;
use ragit_utils::index::commands::archive::archive_create_command;
use ragit_utils::index::commands::archive::archive_extract_command;
use ragit_utils::index::commands::audit::audit_command;
use ragit_utils::index::commands::build::build_command;
use ragit_utils::index::commands::cat_file::cat_file_command;
use ragit_utils::index::commands::check::check_command;
use ragit_utils::index::commands::clone::clone_command;
use ragit_utils::index::commands::config::config_command;
use ragit_utils::index::commands::extract_keywords::extract_keywords_command;
use ragit_utils::index::commands::gc::gc_command;
use ragit_utils::index::commands::help::help_command;
use ragit_utils::index::commands::ii_build::ii_command;
use ragit_utils::index::commands::init::init_command;
use ragit_utils::index::commands::ls::ls_chunks_command;
use ragit_utils::index::commands::ls::ls_files_command;
use ragit_utils::index::commands::ls::ls_images_command;
use ragit_utils::index::commands::ls::ls_models_command;
use ragit_utils::index::commands::ls::ls_terms_command;
use ragit_utils::index::commands::merge::merge_command;
use ragit_utils::index::commands::meta::meta_command;
use ragit_utils::index::commands::migrate::migrate_command;
use ragit_utils::index::commands::model::model_command;
use ragit_utils::index::commands::muse_summarize::muse_summarize_command;
use ragit_utils::index::commands::pdl::pdl_command;
use ragit_utils::index::commands::pull::pull_command;
use ragit_utils::index::commands::push::push_command;
use ragit_utils::index::commands::qa_test::qa_test_command;
use ragit_utils::index::commands::qa_tune::qa_tune_command;
use ragit_utils::index::commands::query::query_command;
use ragit_utils::index::commands::remove::remove_command;
use ragit_utils::index::commands::status::status_command;
use ragit_utils::index::commands::summary::summary_command;
use ragit_utils::index::commands::version::version_command;
use ragit_utils::index::load_mode::LoadMode;
use ragit_utils::index::index_struct::Index;
use async_recursion::async_recursion;

#[async_recursion(?Send)]
pub async fn run(args: Vec<String>) -> Result<(), Error> {
    let (args, pre_args) = ragit_cli::parse_pre_args(&args)?;

    if let Some(path) = pre_args.arg_flags.get("-C") {
        std::env::set_current_dir(path)?;
    }

    let command = args.get(1).map(|arg| arg.as_str());

    match command {
        Some("add") => add_command(args, pre_args).await?,
        Some("archive") => {
            let command = args.get(2).map(|arg| arg.as_str());

            match command {
                Some("create") => archive_create_command(args, pre_args).await?,
                Some("extract") => archive_extract_command(args, pre_args).await?,
                _ => {
                    return Err(Error::CliError {
                        message: String::from("Unknown archive command."),
                        span: (String::new(), 0, 0),
                    });
                }
            }
        },
        Some("audit") => audit_command(args, pre_args).await?,
        Some("build") => build_command(args, pre_args).await?,
        Some("cat-file") => cat_file_command(args, pre_args).await?,
        Some("check") => check_command(args, pre_args).await?,
        Some("clone") => clone_command(args, pre_args).await?,
        Some("config") => config_command(args, pre_args).await?,
        Some("extract-keywords") => extract_keywords_command(args, pre_args).await?,
        Some("gc") => gc_command(args, pre_args).await?,
        Some("help") => help_command(args, pre_args).await?,
        Some("ii") => ii_command(args, pre_args).await?,
        Some("init") => init_command(args, pre_args).await?,
        Some("ls") => {
            let command = args.get(2).map(|arg| arg.as_str());

            match command {
                Some("chunks") => ls_chunks_command(args, pre_args).await?,
                Some("files") => ls_files_command(args, pre_args).await?,
                Some("images") => ls_images_command(args, pre_args).await?,
                Some("models") => ls_models_command(args, pre_args).await?,
                Some("terms") => ls_terms_command(args, pre_args).await?,
                _ => {
                    return Err(Error::CliError {
                        message: String::from("Unknown ls command."),
                        span: (String::new(), 0, 0),
                    });
                }
            }
        },
        Some("merge") => merge_command(args, pre_args).await?,
        Some("meta") => meta_command(args, pre_args).await?,
        Some("migrate") => migrate_command(args, pre_args).await?,
        Some("model") => model_command(args, pre_args).await?,
        Some("muse-summarize") => muse_summarize_command(args, pre_args).await?,
        Some("pdl") => pdl_command(args, pre_args).await?,
        Some("pull") => pull_command(args, pre_args).await?,
        Some("push") => push_command(args, pre_args).await?,
        Some("qa-test") => qa_test_command(args, pre_args).await?,
        Some("qa-tune") => qa_tune_command(args, pre_args).await?,
        Some("query") => query_command(args, pre_args).await?,
        Some("remove") => remove_command(args, pre_args).await?,
        Some("status") => status_command(args, pre_args).await?,
        Some("summary") => summary_command(args, pre_args).await?,
        Some("version") => version_command(args, pre_args).await?,
        _ => {
            return Err(Error::CliError {
                message: String::from("Unknown command."),
                span: (String::new(), 0, 0),
            });
        }
    }

    Ok(())
}