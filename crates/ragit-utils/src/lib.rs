pub mod uid;

pub use uid::*;

use std::path::{Path, PathBuf};

/// Converts a `PathBuf` to an `&str`. Panics if the path is not valid UTF-8.
pub fn pathbuf_to_str(path: &PathBuf) -> &str {
    path.to_str().expect("Path is not valid UTF-8")
}

/// Converts an `&str` to a `PathBuf`.
pub fn str_to_pathbuf(s: &str) -> PathBuf {
    PathBuf::from(s)
}

/// Returns a displayable object for a `Path`.
pub fn path_to_display<'a>(path: &'a Path) -> impl std::fmt::Display + 'a {
    path.display()
}

/// Converts an `&str` to an `&Path`.
pub fn str_to_path_ref(s: &str) -> &Path {
    Path::new(s)
}
