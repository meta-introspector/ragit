use crate::prelude::*;

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if let Err(e) = run(args.clone()).await {
        handle_error(e);
        std::process::exit(1);
    }
}

#[async_recursion(?Send)]
async fn run(args: Vec<String>) -> Result<(), Error> {
    let (args, pre_args) = parse_pre_args(&args)?;

    if let Some(path) = pre_args.arg_flags.get("-C") {
        std::env::set_current_dir(path)?;
    }

    let command = args.get(1).map(|arg| arg.as_str());

    match command {
        use ragit_utils::index::commands::add::add_command;
use ragit_utils::index::commands::archive::create::archive_create_command;
use ragit_utils::index::commands::archive::extract::archive_extract_command;
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
use ragit_utils::index::commands::version::version_command;,
        _ => {
            return Err(Error::CliError {
                message: String::from("Unknown command."),
                span: (String::new(), 0, 0),
            });
        }
    }

    Ok(())
}

fn find_root() -> Result<PathBuf, Error> {
    let current_dir = std::env::current_dir()?;
    let mut current_dir: PathBuf = current_dir.to_path_buf();

    loop {
        if exists(&join(&current_dir, INDEX_DIR_NAME)?) {
            return Ok(current_dir);
        }

        if let Some(parent_dir) = current_dir.parent() {
            current_dir = parent_dir.to_path_buf();
        } else {
            return Err(Error::IndexNotFound);
        }
    }
}

fn handle_error(e: Error) {
    match e {
        Error::IndexNotFound => {
            eprintln!("`.ragit/` not found. Make sure that it's a valid ragit repo.");
        }
        Error::InvalidConfigKey(k) => {
            let similar_key = match find_root() {
                Ok(root) => match Index::load(root, LoadMode::OnlyJson) {
                    Ok(index) => match index.get_all_configs() {
                        Ok(configs) => get_closest_string(
                            &configs.iter().map(|(key, _)| key.to_string()).collect::<Vec<_>>(),
                            &k,
                        ),
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            };

            eprintln!(
                "{k:?} is not a valid key for config.{}",
                if let Some(similar_key) = similar_key {
                    format!(" There is a similar key: `{similar_key}`.")
                } else {
                    String::new()
                },
            );
        }
        Error::FeatureNotEnabled { action, feature } => {
            eprintln!("In order to {action}, you have to enable feature {feature}.");
        }
        Error::InvalidModelName { name, candidates } => {
            if candidates.is_empty() {
                let all_model_names = if let Ok(root_dir) = find_root() {
                    if let Ok(index) = Index::load(root_dir, LoadMode::OnlyJson) {
                        index.models.iter().map(|model| model.name.to_string()).collect::<Vec<_>>()
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                };

                eprintln!(
                    "No model matches `{name}`. Valid model names are: {}",
                    all_model_names.join(", ")
                );
            } else {
                eprintln!(
                    "There are multiple models that match `{name}`: {}",
                    candidates.join(", ")
                );
            }
        }
        Error::DeprecatedConfig { key, message } => {
            eprintln!("Config `{key}` is deprecated!\n{message}");
        }
        Error::CliError { message, span } => {
            eprintln!(
                "cli error: {message}\n\n{}",
                ragit_cli::underline_span(&span.0, span.1, span.2,)
            );
        }
        Error::DirtyKnowledgeBase => {
            eprintln!("The knowledge-base is dirty. Run `rag check --recover`.");
        }
        e => {
            eprintln!("{e:?}");
        }
    }
}