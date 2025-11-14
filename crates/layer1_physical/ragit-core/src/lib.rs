use std::path::Path;

pub trait Matcher {
    fn is_match(&self, path: &Path) -> bool;
}
