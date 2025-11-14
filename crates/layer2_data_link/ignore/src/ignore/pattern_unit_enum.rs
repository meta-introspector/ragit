use regex::Regex;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum PatternUnit {
    DoubleAster,
    // **
    Regex(Regex),
    // a*
    Fixed(String),
    // a
}

impl FromStr for PatternUnit {
    type Err = regex::Error;
    fn from_str(s: &str) -> Result<Self, regex::Error> {
        if s == "**" {
            Ok(PatternUnit::DoubleAster)
        } else if s.contains("*") || s.contains("?") || s.contains("[") {
            let s = s
                .replace(".", "\\.")
                .replace("+", "\\+")
                .replace("(", "\\(")
                .replace(")", "\\)")
                .replace("{", "\\{")
                .replace("}", "\\}")
                .replace("*", ".*")
                .replace("?", ".");
            Ok(PatternUnit::Regex(Regex::new(&format!("^{s}$"))?))
        } else {
            Ok(PatternUnit::Fixed(s.to_string()))
        }
    }
}
