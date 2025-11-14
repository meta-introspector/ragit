use crate::Pattern;
use ragit_core::Matcher;
use ragit_core_utils::path::get_relative_path;
use ragit_fs::{is_dir, is_symlink, read_dir};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Ignore {
    pub patterns: Vec<Pattern>,
    /// Some patterns are stronger than others. For example, you cannot `rag add .ragit/` even with `--force`.
    pub strong_patterns: Vec<Pattern>,
}

impl Default for Ignore {
    fn default() -> Self {
        Self::new()
    }
}

impl Ignore {
    pub fn new() -> Self {
        Ignore {
            patterns: vec![],
            strong_patterns: vec![],
        }
    }

    pub fn add_line(&mut self, line: &str) {
        if !line.is_empty() && !line.starts_with("#") {
            self.patterns.push(Pattern::parse(line));
        }
    }

    pub fn add_strong_pattern(&mut self, pattern: &str) {
        self.strong_patterns.push(Pattern::parse(pattern));
    }

    // like `.gitignore`, `.ragignore` never fails to parse
    pub fn parse(s: &str) -> Self {
        let mut patterns = vec![];
        for line in s.lines() {
            let t = line.trim();
            if t.is_empty() || t.starts_with("#") {
                continue;
            }
            patterns.push(Pattern::parse(t));
        }
        Ignore {
            patterns,
            strong_patterns: vec![],
        }
    }

    /// It returns `Vec<(ignored: bool, file: String)>`. It only returns files, not dirs.
    pub fn walk_tree(
        &self,
        root_dir: &Path,
        dir: &Path,
        follow_symlink: bool,
        skip_ignored_dirs: bool,
    ) -> Result<Vec<(bool, String)>, std::io::Error> {
        let mut result = vec![];
        self.walk_tree_worker(
            root_dir,
            dir,
            &mut result,
            follow_symlink,
            skip_ignored_dirs,
            false,
        )?;
        Ok(result)
    }

    fn walk_tree_worker(
        &self,
        root_dir: &Path,
        file: &Path,
        buffer: &mut Vec<(bool, String)>,
        follow_symlink: bool,
        skip_ignored_dirs: bool,
        already_ignored: bool,
    ) -> Result<(), std::io::Error> {
        // if a file is inside an ignored directory, there's no need to call `is_match` again
        if self.is_strong_match(root_dir, file) {
            return Ok(());
        }

        // ragit doesn't track sym links at all
        if is_symlink(file.to_str().unwrap()) && !follow_symlink {
            return Ok(());
        }
        let is_match = already_ignored || self.is_match(file);
        if is_dir(file.to_str().unwrap()) {
            if !skip_ignored_dirs || !is_match {
                for entry in read_dir(file.to_str().unwrap(), false)? {
                    self.walk_tree_worker(
                        root_dir,
                        &PathBuf::from(entry),
                        buffer,
                        follow_symlink,
                        skip_ignored_dirs,
                        is_match,
                    )?;
                }
            }
        } else {
            buffer.push((is_match, file.to_str().unwrap().to_string()));
        }
        Ok(())
    }

    /// Some patterns are stronger than others. For example, you cannot `rag add .ragit/` even with `--force`.
    pub fn is_strong_match(&self, root_dir: &Path, file: &Path) -> bool {
        let Ok(rel_path) = get_relative_path(root_dir, file) else {
            return false;
        };
        for pattern in self.strong_patterns.iter() {
            if pattern.is_match(&rel_path) {
                return true;
            }
        }
        false
    }
}

impl Matcher for Ignore {
    fn is_match(&self, path: &Path) -> bool {
        for pattern in self.patterns.iter() {
            if pattern.is_match(path) {
                return true;
            }
        }
        false
    }
}
