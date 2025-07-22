use std::path::{Path, PathBuf};

/// Converts a `PathBuf` to an `&str`. Panics if the path is not valid UTF-8.
pub fn pathbuf_to_str(path: &PathBuf) -> String {
    path.to_str().expect("Path is not valid UTF-8").to_string()
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

/// Joins a child path to a base path.
pub fn join_paths(base: &Path, child: &Path) -> PathBuf {
    base.join(child)
}

/// Joins three path components.
pub fn join3_paths(p1: &Path, p2: &Path, p3: &Path) -> PathBuf {
    p1.join(p2).join(p3)
}

/// Normalizes a path, resolving `.` and `..` components.
/// This is a simplified version and might not handle all edge cases like `fs::canonicalize`.
pub fn normalize_path(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ std::path::Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            std::path::Component::Prefix(..) => unreachable!(),
            std::path::Component::RootDir => {
                ret.push(component.as_os_str());
            }
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                ret.pop();
            }
            std::path::Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}

/// Converts a relative path to an absolute path.
/// This is a simplified version and might not handle all edge cases like `fs::canonicalize`.
pub fn to_absolute_path(path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        current_dir.join(path)
    }
}