use crate::PatternUnit;

pub fn match_dir(pattern: &PatternUnit, path: &str) -> bool {
    match pattern {
        PatternUnit::DoubleAster => true,
        PatternUnit::Regex(r) => r.is_match(path),
        PatternUnit::Fixed(p) => path == p,
    }
}
