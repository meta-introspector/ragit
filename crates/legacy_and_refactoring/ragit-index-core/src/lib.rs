pub mod add_files;
pub use add_files::add_files_command;
pub mod clone;
pub use clone::clone_command;

pub mod prelude;
pub use ragit_index_effects::build;
// pub use ragit_index_save_to_file::load_index_from_path; // FIXME: load_index_from_path not found
pub use ragit_index_types::{Index, load_mode::LoadMode};

