use ragit_core::Matcher;
use ragit_core_utils::path::get_relative_path;
use ragit_fs::{is_dir, is_symlink, read_dir};
use std::path::{Path, PathBuf};

mod ignore;
pub use ignore::ignore_struct::Ignore;
pub use ignore::match_dir_fn::match_dir;
pub use ignore::match_worker_fn::match_worker;
pub use ignore::pattern_struct::Pattern;
pub use ignore::pattern_unit_enum::PatternUnit;
