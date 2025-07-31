use super::match_dir_fn::match_dir;
use crate::PatternUnit;

pub fn match_worker(pattern: Vec<PatternUnit>, path: Vec<String>) -> bool {
    // (0, 0) means it's looking at pattern[0] and path[0].
    // if it reaches (pattern.len(), path.len()), it matches
    let mut cursors = vec![(0, 0)];
    while let Some((pattern_cursor, path_cursor)) = cursors.pop() {
        if pattern_cursor == pattern.len() && path_cursor == path.len() {
            return true;
        }
        if pattern_cursor >= pattern.len() || path_cursor >= path.len() {
            if let Some(PatternUnit::DoubleAster) = pattern.get(pattern_cursor) {
                if !cursors.contains(&(pattern_cursor + 1, path_cursor)) {
                    cursors.push((pattern_cursor + 1, path_cursor));
                }
            }
            continue;
        }
        if match_dir(&pattern[pattern_cursor], &path[path_cursor]) {
            if let PatternUnit::DoubleAster = &pattern[pattern_cursor] {
                if !cursors.contains(&(pattern_cursor, path_cursor + 1)) {
                    cursors.push((pattern_cursor, path_cursor + 1));
                }
                if !cursors.contains(&(pattern_cursor + 1, path_cursor)) {
                    cursors.push((pattern_cursor + 1, path_cursor));
                }
            }
            if !cursors.contains(&(pattern_cursor + 1, path_cursor + 1)) {
                cursors.push((pattern_cursor + 1, path_cursor + 1));
            }
        }
    }
    false
}
