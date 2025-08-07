pub mod process_blob;
pub mod process_commit;
pub mod extract_terms_from_content;
pub mod config_loader;
pub mod repo_processor;
pub mod finalizer;
pub mod run_update_command;
pub mod commit_iterator;
pub mod commit_reporter;

pub use run_update_command::run_update_command;