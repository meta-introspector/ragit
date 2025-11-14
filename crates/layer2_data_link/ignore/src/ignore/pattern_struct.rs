use super::match_worker_fn::match_worker;
use crate::PatternUnit;
use ragit_core::Matcher;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Pattern(pub Vec<PatternUnit>);

impl Pattern {
    pub fn parse(pattern: &str) -> Self {
        let mut pattern = pattern.to_string();

        // `a/b` -> `**/a/b`
        // `/a/b` -> `a/b`
        if !pattern.starts_with("/") {
            pattern = format!("**/{pattern}");
        } else {
            pattern = pattern.get(1..).unwrap().to_string();
        }

        // I'm not sure about this...
        if pattern.ends_with("/") {
            pattern = pattern.get(0..(pattern.len() - 1)).unwrap().to_string();
        }
        let mut result = pattern
            .split("/")
            .map(|p| {
                p.parse::<PatternUnit>()
                    .unwrap_or_else(|_| PatternUnit::Fixed(p.to_string()))
            })
            .collect::<Vec<_>>();
        match result.last() {
            Some(PatternUnit::DoubleAster) => {}
            _ => {
                // `target` must match `crates/ignore/target/debug`
                result.push(PatternUnit::DoubleAster);
            }
        }
        Pattern(result)
    }
}

impl Matcher for Pattern {
    // `path` must be a normalized, relative path
    fn is_match(&self, path: &Path) -> bool {
        let mut path_str = path.to_str().unwrap().to_string();

        // there's no reason to treat `a/b` and `a/b/` differently
        if path_str.len() > 1 && path_str.ends_with("/") {
            path_str = path_str.get(0..(path_str.len() - 1)).unwrap().to_string();
        }
        match_worker(
            self.0.clone(),
            path_str
                .split("/")
                .map(|p| p.to_string())
                .collect::<Vec<_>>(),
        )
    }
}
