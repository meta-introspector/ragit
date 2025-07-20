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
        Some("add") => crate::index::commands::add_command::add_command(&args).await?,
        Some("archive-create") | Some("create-archive") | Some("archive") => crate::index::commands::archive_create_command::archive_create_command(&args).await?,
        Some("archive-extract") | Some("extract-archive") | Some("extract") => crate::index::commands::archive_extract_command::archive_extract_command(&args).await?,
        Some("audit") => crate::index::commands::audit_command::audit_command(&args).await?,
        Some("build") => crate::index::commands::build_command::build_command(&args).await?,
        Some("cat-file") => crate::index::commands::cat_file_command::cat_file_command(&args).await?,
        Some("check") => crate::index::commands::check_command::check_command(&args).await?,
        Some("clone") => crate::index::commands::clone_command::clone_command(&args).await?,
        Some("config") => crate::index::commands::config_command::config_command(&args)?,
        Some("extract-keywords") => crate::index::commands::extract_keywords_command::extract_keywords_command(&args).await?,
        Some("gc") => crate::index::commands::gc_command::gc_command(&args)?,
        Some("help") | Some("--help") => crate::index::commands::help_command::help_command(&args).await?,
        Some("ii-build") | Some("build-ii") | Some("ii-reset") | Some("reset-ii") | Some("ii-status") => crate::index::commands::ii_command::ii_command(&args)?,
        Some("init") => crate::index::commands::init_command::init_command(&args)?,
        Some("ls-chunks") => crate::index::commands::ls_chunks_command::ls_chunks_command(&args)?,
        Some("ls-files") => crate::index::commands::ls_files_command::ls_files_command(&args)?,
        Some("ls-images") => crate::index::commands::ls_images_command::ls_images_command(&args)?,
        Some("ls-models") => crate::index::commands::ls_models_command::ls_models_command(&args)?,
        Some("ls-terms") => crate::index::commands::ls_terms_command::ls_terms_command(&args)?,
        Some("merge") => crate::index::commands::merge_command::merge_command(&args)?,
        Some("meta") => crate::index::commands::meta_command::meta_command(&args)?,
        Some("migrate") => crate::index::commands::migrate_command::migrate_command(&args)?,
        Some("model") => crate::index::commands::model_command::model_command(&args).await?,
        Some("muse-summarize") => crate::index::commands::muse_summarize_command::muse_summarize_command(&args).await?,
        Some("pdl") => crate::index::commands::pdl_command::pdl_command(&args).await?,
        Some("pull") => crate::index::commands::pull_command::pull_command(&args).await?,
        Some("push") => crate::index::commands::push_command::push_command(&args).await?,
        Some("qa-test") => crate::index::commands::qa_test_command::qa_test_command(&args).await?,
        Some("qa-tune") => crate::index::commands::qa_tune_command::qa_tune_command(&args).await?,
        Some("query") => crate::index::commands::query_command::query_command(&args).await?,
        Some("remove") => crate::index::commands::remove_command::remove_command(&args)?,
        Some("status") => crate::index::commands::status_command::status_command(&args)?,
        Some("summary") => crate::index::commands::summary_command::summary_command(&args).await?,
        Some("version") => crate::index::commands::version_command::version_command(&args)?,
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