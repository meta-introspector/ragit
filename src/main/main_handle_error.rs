use crate::prelude::*;
use crate::main::main_find_root::find_root;
use ragit_utils::index::index_struct::Index;
use ragit_utils::index::load_mode::LoadMode;
use ragit_utils::string_utils::get_closest_string;
use ragit_cli::underline_span;

pub fn handle_error(e: Error) {
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
                }
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
        Error::CliError(cli_error) => {
            eprintln!(
                "cli error: {}\n\n{}",
                cli_error.to_string(),
                ragit_cli::underline_span(&cli_error.get_span().unwrap_rendered().0, cli_error.get_span().unwrap_rendered().1, cli_error.get_span().unwrap_rendered().2,)
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