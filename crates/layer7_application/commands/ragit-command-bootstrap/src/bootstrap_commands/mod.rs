pub mod setup_environment;
pub mod copy_prompts;
pub mod add_bootstrap_files;
pub mod build_index_logic;
pub mod process_staged_file_logic;
pub mod write_chunks_to_markdown;
pub mod perform_self_improvement;
pub mod perform_final_reflective_query;
pub mod constants;
pub use crate::memory_utils;

pub mod self_improvement;
pub mod final_reflective_query;
pub mod configure_memory_settings;
