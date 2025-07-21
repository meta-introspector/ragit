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

fn find_root() -> Result<PathBuf, Error> {
    let current_dir = std::env::current_dir()?;
    let mut current_dir: PathBuf = current_dir.to_path_buf();

    loop {
        if ragit_fs::exists(&ragit_fs::join(&current_dir.to_string_lossy().into_owned(), INDEX_DIR_NAME)?) {
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
                        Ok(configs) => ragit_utils::string_utils::get_closest_string(
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
